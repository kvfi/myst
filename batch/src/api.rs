use std::collections::HashMap;

use chrono::Utc;
use diesel::SqliteConnection;
use log::{error, info};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::Config;
use crate::db;

#[derive(Deserialize)]
struct RequestCodeResponse {
    code: String,
}

#[derive(Deserialize)]
struct RequestTokenResponse {
    access_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorSearchMeta {
    search_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkItemResponse {
    pub(crate) item_id: String,
    resolved_id: String,
    given_url: String,
    given_title: String,
    favorite: String,
    status: String,
    pub(crate) time_added: String,
    time_updated: String,
    time_read: String,
    time_favorited: String,
    sort_id: u16,
    pub(crate) resolved_title: Option<String>,
    pub(crate) resolved_url: Option<String>,
    excerpt: Option<String>,
    is_article: Option<String>,
    is_index: Option<String>,
    has_video: Option<String>,
    has_image: Option<String>,
    word_count: Option<String>,
    lang: Option<String>,
    listen_duration_estimate: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LinkListResponse {
    Error {
        status: u32,
        complete: u32,
        list: Vec<String>,
        error: Option<String>,
        search_meta: ErrorSearchMeta,
        since: u64,
    },
    Success {
        status: u8,
        list: HashMap<String, LinkItemResponse>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub status: u32,
    pub details: String,
}

#[tokio::main]
pub(crate) async fn obtain_request_code(cfg: &mut Config) -> Result<(), reqwest::Error> {
    let payload = json!({
        "consumer_key": cfg.consumer_key,
        "redirect_uri": cfg.redirect_url
    });

    let request_url: &str = &format!("{}/oauth/request", cfg.api_endpoint);
    let response = Client::new()
        .post(request_url)
        .header("X-Accept", "application/json")
        .json(&payload)
        .send()
        .await?;

    let resp: RequestCodeResponse = response.json().await?;
    cfg.code = Option::from(resp.code);

    Ok(())
}

pub(crate) fn authorize_app(config: &mut Config) {
    let auth_url: String = format!(
        "https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
        config.code.as_ref().unwrap(),
        config.redirect_url
    );
    let mut buffer = String::new();

    config.auth_url = Option::from(auth_url.clone());
    info!(
        "Open the following URL in your browser and click on authorize: {}",
        auth_url
    );

    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => println!("{}", buffer),
        Err(e) => println!("{}", e),
    }

    drop(auth_url);
}

#[tokio::main]
pub(crate) async fn obtain_request_token(config: &mut Config) -> Result<(), reqwest::Error> {
    let payload = json!({
        "consumer_key": config.consumer_key,
        "code": config.code
    });

    let request_url = format!("{}/oauth/authorize", config.api_endpoint);
    let response = Client::new()
        .post(request_url)
        .header("X-Accept", "application/json")
        .json(&payload)
        .send()
        .await?;
    let resp: RequestTokenResponse = response.json().await?;
    config.token = Option::from(resp.access_token);

    Ok(())
}

#[tokio::main]
pub(crate) async fn code_is_valid(config: &mut Config) {
    match &config.auth_url {
        Some(url) => match Client::new().get(url).send().await {
            Ok(resp) => {
                let status: StatusCode = resp.status();
                config.code_valid = Option::from(status.is_success());
            }
            Err(e) => error!("Error validating auth URL: {}", e),
        },
        None => {
            config.code_valid = Option::from(false);
            error!("Auth URL is not specified")
        }
    }
}

#[tokio::main]
pub(crate) async fn obtain_links(
    conn: &mut SqliteConnection,
    config: &mut Config,
) -> Result<Vec<LinkItemResponse>, ApiError> {
    let payload = json!({
        "consumer_key": config.consumer_key,
        "access_token": config.token,
        "sort": "newest",
        "state": "all",
        "since": config.last_retrieval,
        "count": u32::MAX
    });

    let request_url = format!("{}/get", config.api_endpoint);
    match Client::new()
        .post(request_url)
        .header("X-Accept", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            let result = response.text().await.unwrap();
            let link_list_response: LinkListResponse = serde_json::from_str(&result).unwrap();
            match link_list_response {
                LinkListResponse::Success { list, .. } => {
                    let links: Vec<LinkItemResponse> = list.values().cloned().collect();
                    config.last_retrieval = Option::from(Utc::now().timestamp());
                    db::store_pocket_links(conn, &links);
                    config.save();
                    Ok(links)
                }
                LinkListResponse::Error { status, .. } => Err(ApiError {
                    status,
                    details: "Could not retrieve data.".to_string(),
                }),
            }
        }
        Err(e) => Err(ApiError {
            status: 503,
            details: format!("Remove service not available. {:?}", e),
        }),
    }
}
