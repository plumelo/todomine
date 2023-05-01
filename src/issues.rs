use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use todo_txt::Task;

use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE},
};

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
    description: String,
}

#[derive(Deserialize, Debug)]
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
    project_id: Option<String>,
    status_id: String,
    assigned_to_id: Option<String>,
}
impl Default for Filter {
    fn default() -> Self {
        Self {
            offset: None,
            limit: None,
            sort: None,
            issue_id: vec![],
            project_id: None,
            status_id: "*".to_string(),
            assigned_to_id: None,
        }
    }
}

impl Issue {
    pub fn into_task(self) -> Task {
        let mut tags = BTreeMap::new();
        tags.insert("rid".to_string(), self.id.to_string());
        Task {
            subject: self.subject,
            tags,
            projects: vec![self.project.name],
            ..Task::default()
        }
    }
}
impl Issues {
    pub fn into_tasks(self) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
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
        let url = vec![self.url, "issues.json".to_string()].join("/");
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            //.query(&self.filter)
            .header("X-Redmine-API-Key", self.key)
            .send()
            .await?;
        let issues = res.json::<Issues>().await?;
        Ok(issues)
    }
}
