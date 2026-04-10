use directories::ProjectDirs;
use reqwest::cookie::CookieStore;
use reqwest::{Url, header::HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub(crate) struct Config {
    #[serde(flatten)]
    pub(crate) cookie: Option<Cookie>,
    pub(crate) ipb_member_id: Option<String>,
    #[serde(skip_serializing)]
    pub(crate) key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Cookie {
    pub(crate) ipb_pass_hash: String,
    pub(crate) sk: String,
    pub(crate) igneous: String,
}

impl Config {
    pub(crate) fn load() -> Self {
        ProjectDirs::from("moe", "Assistant", "sadpanda")
            .map(|d| d.config_dir().join("config.toml"))
            .and_then(|f| read_to_string(&f).ok())
            .and_then(|s| toml::from_str::<Config>(&s).ok())
            .unwrap_or_default()
    }
}

impl CookieStore for Config {
    fn set_cookies(&self, _: &mut dyn Iterator<Item = &reqwest::header::HeaderValue>, _: &Url) {}

    fn cookies(&self, _url: &Url) -> Option<reqwest::header::HeaderValue> {
        serde_urlencoded::to_string(self)
            .ok()
            .map(|s| s.replace('&', "; "))
            .and_then(|s| HeaderValue::from_str(&s).ok())
    }
}
