use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use todo_txt::Task;

#[derive(Deserialize, Debug)]
struct Project {
    id: u16,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Status {
    id: u16,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Issue {
    id: u16,
    project: Project,
    status: Status,
    subject: String,
}

#[derive(Deserialize, Debug)]
pub struct Issues {
    issues: Vec<Issue>,
    total_count: u16,
    offset: u16,
    limit: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Filter {
    offset: Option<u16>,
    limit: Option<u16>,
    sort: Option<String>,
    #[serde(rename = "status_id")]
    status: String,
    #[serde(rename = "project_id")]
    project: Option<String>,
    #[serde(rename = "assigned_to_id")]
    assigned_to: Option<String>,
}
impl Default for Filter {
    fn default() -> Self {
        Self {
            offset: None,
            limit: None,
            sort: None,
            status: "*".to_string(),
            project: None,
            assigned_to: None,
        }
    }
}

impl Issue {
    pub fn into_task(self) -> Task {
        let mut tags = BTreeMap::new();
        tags.insert("rid".to_string(), self.id.to_string());
        Task {
            subject: self.subject,
            finished: self.status.name == "Closed",
            tags,
            projects: vec![self.project.name],
            ..Task::default()
        }
    }
}
impl Issues {
    pub fn into_tasks(mut self) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
        self.issues.sort_by(|a, b| b.status.id.cmp(&a.status.id));

        for issue in self.issues {
            tasks.push(issue.into_task());
        }
        tasks
    }
}

pub struct ListIssues {
    url: String,
    key: String,
    filter: Filter,
}

impl ListIssues {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            key,
            filter: Filter::default(),
        }
    }

    pub async fn get(self) -> reqwest::Result<Issues> {
        let url = format!("{}/issues.json", self.url);
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .query(&self.filter)
            .header("X-Redmine-API-Key", self.key)
            .send()
            .await?;
        let issues = res.json::<Issues>().await?;
        Ok(issues)
    }
}
