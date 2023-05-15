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
pub struct Issue {
    pub id: u16,
    project: Project,
    status: Status,
    subject: String,
}

impl Issue {
    pub fn into_task(&self) -> Task {
        let mut tags = BTreeMap::new();
        let mut task = Task::default();
        tags.insert("rid".to_string(), self.id.to_string());
        task.tags = tags;
        self.sync_task(&mut task);
        task
    }
    pub fn sync_task(&self, task: &mut Task) {
        task.subject = self.subject.clone();
        task.finished = self.status.name == "Closed";
        task.projects = vec![self.project.name.to_lowercase()];
    }
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

#[derive(Deserialize, Debug)]
pub struct Issues {
    issues: Vec<Issue>,
    //total_count: u16,
    //offset: u16,
    //limit: u16,
}

pub struct ListIssues {
    url: String,
    key: String,
    filter: Filter,
}

impl ListIssues {
    pub fn new(url: String, key: String, project: Option<String>) -> Self {
        Self {
            url,
            key,
            filter: Filter {
                project,
                ..Filter::default()
            },
        }
    }

    pub async fn get(self) -> reqwest::Result<Vec<Issue>> {
        let url = format!("{}/issues.json", self.url);
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .query(&self.filter)
            .header("X-Redmine-API-Key", self.key)
            .send()
            .await?;
        Ok(res.json::<Issues>().await?.issues)
    }
}
