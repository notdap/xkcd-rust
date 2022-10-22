use lazy_static::lazy_static;

use regex::Regex;

use anyhow::{Result, Context, Ok, bail};

use reqwest::Client;
use reqwest::header::USER_AGENT;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use http_cache_reqwest::{Cache, HttpCache, CacheMode, CACacheManager};

use serde_json::Value;

lazy_static! {
    static ref AGENT: String = String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.126 Safari/537.36");
    static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();
}

pub async fn get_json_from_url(url: &str) -> Result<Value> {
    let response = CLIENT
        .get(url.to_owned())
        .header(USER_AGENT, AGENT.as_str())
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str(&response) {
        core::result::Result::Ok(val) => Ok(val),
        core::result::Result::Err(err) => bail!(err),
    }
}

pub async fn get_xkcd_api_url_from_string(query: &str) -> Result<String> {
    let response = CLIENT
        .get("https://html.duckduckgo.com/html/?q=site:xkcd.com+".to_string() + &query.to_string().replace(" ", "+"))
        .header(USER_AGENT, AGENT.as_str())
        .send()
        .await?
        .text()
        .await?;

    let regex = Regex::new(r"xkcd.com/\d+/?")?;
    let parsed = &regex.captures(&response).context("No matches found")?[0];

    let result = format!("https://www.{}info.0.json", parsed);

    Ok(result)
}

pub fn get_xkcd_api_url_from_int(query: &u32) -> String {
    format!("https://www.xkcd.com/{}/info.0.json", query)
}