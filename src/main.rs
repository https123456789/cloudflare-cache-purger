use anyhow::{bail, Context, Result};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

const API_BASE: &str = "https://api.cloudflare.com";

#[derive(Debug, Serialize)]
struct CachePurgeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    purge_everything: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ResponseInfo {
    code: usize,
    message: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CachePurgeResponse {
    errors: Vec<ResponseInfo>,
    success: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_token = std::env::var("CLOUDFLARE_API_TOKEN")
        .context("Missing CLOUDFLARE_API_TOKEN environment variable")?;
    let zone_id = std::env::var("CLOUDFLARE_ZONE_ID")
        .context("Missing CLOUDFLARE_ZONE_ID environment variable")?;

    let purge_targets: Vec<String> = std::env::args()
        .skip(1)
        .map(|item| item.to_owned())
        .collect();
    let mut data = CachePurgeData {
        files: Some(purge_targets.clone()),
        purge_everything: None,
    };

    // Handle the "purge it all" option
    if !purge_targets.is_empty() && purge_targets[0] == "--purge-all" {
        data = CachePurgeData {
            files: None,
            purge_everything: Some(true),
        };
        println!("Purging everything");
    } else {
        println!("Purging the following:");
        for item in purge_targets {
            println!("- {item}");
        }
    }

    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {api_token}"))?,
    );

    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .build()?;

    let resp = client
        .post(format!("{API_BASE}/client/v4/zones/{zone_id}/purge_cache"))
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )
        .json::<CachePurgeData>(&data)
        .send()
        .await?
        .json::<CachePurgeResponse>()
        .await?;

    if !resp.success {
        for error in resp.errors {
            eprintln!("Error (code {}): {}", error.code, error.message);
        }

        bail!("Request failed!")
    }

    println!("Ok");

    Ok(())
}
