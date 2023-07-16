use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use todo_txt::Task;

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Status {
    is_closed: bool,
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
        task.finished = self.status.is_closed;
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
pub struct ProjectResult {
    id: u16,
    identifier: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectsResult {
    projects: Vec<ProjectResult>,
}

#[derive(Deserialize, Debug)]
pub struct Issues {
    url: String,
    key: String,
    status: Option<String>,
    project: Option<String>,
    limit: u16,
}

impl Issues {
    pub fn new(
        url: String,
        key: String,
        project: Option<String>,
        status: Option<String>,
        limit: u16,
    ) -> Self {
        Self {
            url,
            key,
            project,
            status,
            limit,
        }
    }

    async fn project_id(&self, identifier: String) -> anyhow::Result<u16> {
        reqwest::Client::builder()
            .build()?
            .get(format!("{}/projects.json", self.url.clone()))
            .header("X-Redmine-API-Key", self.key.clone())
            .send()
            .await?
            .json::<ProjectsResult>()
            .await?
            .projects
            .into_iter()
            .find(|p| p.identifier == identifier)
            .map(|p| p.id)
            .ok_or(anyhow!("Could not find project"))
    }

    async fn params(&self) -> anyhow::Result<Vec<(&str, String)>> {
        Ok([
            vec![("limit", self.limit.to_string())],
            (match &self.project {
                Some(identifier) => vec![(
                    "project_id",
                    self.project_id(identifier.clone()).await?.to_string(),
                )],
                None => vec![],
            }),
            (match &self.status {
                Some(status) => vec![("status_id", status.clone())],
                None => vec![],
            }),
        ]
        .concat())
    }

    pub async fn get(&self) -> anyhow::Result<Vec<Issue>> {
        let issues = reqwest::Client::builder()
            .build()?
            .get(format!("{}/issues.json", self.url))
            .header("X-Redmine-API-Key", self.key.clone())
            .query(&(self.params().await?))
            .send()
            .await?
            .json::<IssuesResult>()
            .await?
            .issues;

        Ok(issues)
    }
}
