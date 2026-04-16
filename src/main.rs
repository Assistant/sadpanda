use crate::api::Api;
use clap::Parser;
use dom_query::Document;
use reqwest::blocking::{Client, Response};
use std::{sync::Arc, thread::sleep, time::Duration};

mod api;
mod args;
mod config;

fn main() {
    let args = args::Cli::parse();
    let config = config::Config::load();
    let client = if config.ipb_member_id.is_some() && config.cookie.is_some() {
        Client::builder()
            .cookie_provider(Arc::new(config.clone()))
            .build()
            .unwrap_or_default()
    } else {
        Client::new()
    };

    let mut galleries: Vec<_> = args.urls.iter().filter_map(Gallery::from_url).collect();

    if args.favorites {
        galleries.extend(Gallery::favorites(&client));
    }

    let mut responses = vec![];
    for chunk in galleries.chunks(25) {
        sleep(Duration::from_secs(1));
        if let Some(response) = Api::new(&client, chunk) {
            responses.extend(response.gmetadata);
        }
    }

    for response in responses {
        if args.magnet {
            let magnet = response.magnet(&config).unwrap_or("NA".to_string());
            println!("{magnet} — {}", response.title);
        }
        if args.torrent && response.torrent(&config, &client).is_some() {
            println!("{} torrent downloaded.", response.title);
        }
    }
}

#[derive(Debug)]
struct Gallery {
    id: u64,
    token: String,
}

impl Gallery {
    fn favorites(client: &Client) -> Vec<Gallery> {
        Self::get_galleries(client, "https://exhentai.org/favorites.php")
    }

    fn get_galleries(client: &Client, url: impl AsRef<str>) -> Vec<Gallery> {
        let Ok(Ok(response)) = client.get(url.as_ref()).send().map(Response::text) else {
            return vec![];
        };
        let html = Document::from(response);
        let next = html
            .select_single("a#unext")
            .attr("href")
            .map(|x| x.to_string());
        let thumbs = html.select("a[href*=org\\/g\\/]:has(.glink)");
        let mut links = thumbs
            .iter()
            .filter_map(|a| a.attr("href"))
            .filter_map(Gallery::from_url)
            .collect::<Vec<_>>();
        if let Some(url) = next {
            sleep(Duration::from_secs(1));
            links.extend(Self::get_galleries(client, url));
        }
        links
    }

    fn from_url(url: impl AsRef<str>) -> Option<Self> {
        let mut parts = url.as_ref().split('/').rev().skip(1);
        let (token, id) = (parts.next()?.into(), parts.next()?.parse().ok()?);
        Some(Gallery { id, token })
    }

    fn pair(&self) -> (u64, String) {
        (self.id, self.token.clone())
    }
}
