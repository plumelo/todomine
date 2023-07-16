use serde::Deserialize;
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
            .ok_or(anyhow::anyhow!(
                "Could not find any project with identifier {:?}",
                identifier
            ))
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
