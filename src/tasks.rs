use crate::issues::Issue;
use anyhow::Result;
use std::str::FromStr;
use todo_txt::Task;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct Tasks {
    tasks: Vec<Task>,
    file: String,
}

impl Tasks {
    pub fn new(file: String) -> Self {
        Self {
            file,
            tasks: vec![],
        }
    }
    pub async fn read(mut self) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&self.file)
            .await?;
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await? {
            let task = Task::from_str(line.as_str())?;
            self.tasks.push(task);
        }
        Ok(self)
    }

    pub async fn write(self) -> Result<Self> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file)
            .await?;
        for task in &self.tasks {
            print!("${task}");
            let line = format!("{task}\n");
            file.write(line.as_bytes()).await?;
        }
        Ok(self)
    }

    pub fn sync(mut self, issues: Vec<Issue>) -> Self {
        let mut ids: Vec<u16> = vec![];
        for task in &mut self.tasks {
            let tags = task.tags.to_owned();
            if !tags.contains_key("rid") {
                continue;
            }
            for issue in &issues {
                if let Some(rid) = tags.get("rid") {
                    if issue.id.to_string() == rid.clone() {
                        issue.sync_task(task);
                        ids.push(issue.id.clone());
                    }
                }
            }
        }
        let mut new: Vec<Task> = issues
            .iter()
            .filter(|i| !ids.contains(&i.id))
            .map(|i| i.into_task())
            .collect::<Vec<Task>>();

        new.sort_by(|a, b| a.finished.cmp(&b.finished));

        self.tasks.append(&mut new);
        self
    }
}
