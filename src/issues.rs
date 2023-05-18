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
        task.subject =
            self.subject.to_owned() + &" +".to_string() + &self.project.name.to_lowercase();
        task.finished = self.status.name == "Closed";
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
pub struct IssuesResult {
    issues: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct Issues {
    url: String,
    key: String,
    #[serde(rename = "status_id")]
    status: Option<String>,
    #[serde(rename = "project_id")]
    project: Option<String>,
    #[serde(rename = "assigned_to_id")]
    assigned_to: Option<String>,
}

impl Issues {
    pub fn new(url: String, key: String, project: Option<String>) -> Self {
        Self {
            url,
            key,
            project,
            status: None,
            assigned_to: None,
        }
    }

    pub async fn get(self) -> reqwest::Result<Vec<Issue>> {
        let url = format!("{}/issues.json", self.url);
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .query(&[("project", &self.project)])
            .header("X-Redmine-API-Key", self.key)
            .send()
            .await?;
        Ok(res.json::<IssuesResult>().await?.issues)
    }
}
