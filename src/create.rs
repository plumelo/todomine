use anyhow::{anyhow, Result};
use serde::Deserialize;

use reqwest;

#[derive(Deserialize)]
struct Issue {
    id: u16,
}

#[derive(Deserialize)]
struct Issues {
    issues: Vec<Issue>,
    total_count: u16,
    offset: u16,
    limit: u16,
}

pub async fn create() -> Result<bool> {
    let res = reqwest::get("https://redmine.plumelo.com/issues.json")
        .await?
        .json::<Issues>()
        .await?;

    println!("count: {}", res.total_count);
    Ok(true)
}
