use crate::issues::{Issues, ListIssues};
use reqwest::Result;

pub async fn create() -> Result<Issues> {
    let list = ListIssues::new(
        "https://redmine.plumelo.com".to_string(),
        "fe59cf78192250be66078af4f71a10c925e1b3fa".to_string(),
    )
    .get()
    .await?;
    Ok(list)
}
