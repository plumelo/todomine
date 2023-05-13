use anyhow::Result;
use std::str::FromStr;
use todo_txt::Task;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct Tasks {
    tasks: Vec<Task>,
    file: String,
}

async fn open(file: &String) -> Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file.as_str())
        .await?;
    Ok(file)
}

impl Tasks {
    pub fn new(file: String) -> Self {
        Self {
            file,
            tasks: vec![],
        }
    }
    pub async fn read(mut self) -> Result<Self> {
        let file = open(&self.file).await?;

        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await? {
            let task = Task::from_str(line.as_str())?;
            self.tasks.push(task);
        }
        Ok(self)
    }

    pub async fn write(self) -> Result<Self> {
        let mut file = open(&self.file).await?;
        for task in &self.tasks {
            let line = format!("{task}\n");
            file.write(line.as_bytes()).await?;
        }
        Ok(self)
    }

    pub fn sync(mut self,tasks: &mut Vec<Task>) -> Self {
        self.tasks.append(tasks);
        self
    }
}
