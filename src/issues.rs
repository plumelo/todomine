use serde::{Deserialize, Serialize};

use reqwest;

#[derive(Deserialize)]
struct Issue {
    id: u16,
}

#[derive(Deserialize)]
pub struct Issues {
    issues: Vec<Issue>,
    total_count: u16,
    offset: u16,
    limit: u16,
}

#[derive(Serialize, Clone)]
pub struct Filter {
    offset: Option<u16>,
    limit: Option<u16>,
    sort: Option<String>,

    issue_id: Vec<String>,
    project_id: String,
    status_id: String,
    assign_to: String,
}

pub struct ListIssues {
    url: String,
    key: String,
    filter: Option<Filter>,
}

impl ListIssues {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            key,
            filter: None,
        }
    }

    pub async fn get(&self) -> reqwest::Result<Issues> {
        let url = self.url.to_owned() + "issues.json";
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .query(&self.filter)
            .header("X-Redmine-API-Key", &self.key)
            .send()
            .await?;

        let issues = res.json::<Issues>().await?;
        Ok(issues)
    }
}
