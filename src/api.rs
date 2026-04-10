use crate::{Gallery, config::Config};
use lava_torrent::torrent::v1::Torrent;
use nestify::nest;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

nest! {
    #[allow(dead_code)]*
    #[derive(Deserialize, Debug)]*
    pub(crate) struct Api {
        pub(crate) gmetadata: Vec<pub(crate) struct GalleryData {
            gid: u64,
            token: String,
            pub(crate) title: String,
            title_jpn: String,
            category: String,
            thumb: String,
            uploader: String,
            posted: String,
            filecount: String,
            filesize: u64,
            expunged: bool,
            rating: String,
            torrentcount: String,
            torrents: Vec<struct TorrentData {
                hash: String,
                added: String,
                name: String,
                tsize: String,
                fsize: String,
            }>,
            tags: Vec<String>,
            current_gid: Option<String>,
            current_key: Option<String>,
            parent_gid: Option<String>,
            parent_key: Option<String>,
            first_gid: Option<String>,
            first_key: Option<String>,
        }>
    }
}

impl Api {
    pub(crate) fn new(client: &Client, galleries: &[Gallery]) -> Option<Self> {
        client
            .post("https://api.e-hentai.org/api.php")
            .json(&galleries.request())
            .send()
            .ok()?
            .json::<Self>()
            .ok()
    }
}

impl GalleryData {
    pub(crate) fn tracker(&self, config: &Config) -> String {
        let gallery_id = self.get_gid();
        if let Some(user_id) = &config.ipb_member_id
            && let Some(key) = &config.key
        {
            format!("http://ehtracker.org/{gallery_id}/{user_id}x{key}/announce")
        } else {
            format!("http://ehtracker.org/{gallery_id}/announce")
        }
    }

    pub(crate) fn torrent(&self, config: &Config, client: &Client) -> Option<()> {
        let gid = self.get_gid();
        let hash = self.get_largest_hash()?;
        let url = format!("https://exhentai.org/torrent/{gid}/{hash}.torrent");
        let torrent = client.get(url).send().ok()?.bytes().ok()?;
        let mut torrent = Torrent::read_from_bytes(torrent).ok()?;
        torrent.announce = Some(self.tracker(config));
        torrent.write_into_file(format!("{hash}.torrent")).ok()?;
        Some(())
    }

    pub(crate) fn magnet(&self, config: &Config) -> Option<String> {
        self.get_largest_hash()
            .map(|h| format!("magnet:?xt=urn:btih:{h}&tr={}", self.tracker(config)))
    }

    fn get_gid(&self) -> u64 {
        self.first_gid
            .as_ref()
            .and_then(|g| g.parse().ok())
            .unwrap_or(self.gid)
    }

    fn get_largest_hash(&self) -> Option<&String> {
        self.torrents
            .iter()
            .max_by_key(|x| &x.fsize)
            .map(|t| &t.hash)
    }
}

trait RequestTrait {
    fn request(&self) -> Request;
}

impl RequestTrait for [Gallery] {
    fn request(&self) -> Request {
        Request {
            method: "gdata".into(),
            namespace: 1,
            gidlist: self.iter().map(Gallery::pair).collect(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Request {
    method: String,
    gidlist: Vec<(u64, String)>,
    namespace: usize,
}
