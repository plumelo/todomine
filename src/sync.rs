use crate::issues::ListIssues;
use anyhow::Result;
use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Synchronize Redmine issues with todo.txt files.", long_about = None)]
pub struct Sync {
    /// The todo.txt file to use
    #[arg(short, long, env = "TODOMINE_FILE")]
    file: String,
    /// The Redmine base url
    #[arg(short, long, env = "TODOMINE_REDMINE_API")]
    url: String,
    /// The Redmine api key
    #[arg(short, long, env = "TODOMINE_REDMINE_KEY")]
    key: String,
}

impl Sync {
    pub async fn sync(self) -> Result<()> {
        let list = ListIssues::new(self.url, self.key);
        let tasks = list.get().await?.into_tasks();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.file)
            .await?;
        for task in tasks {
            let line = format!("{task}\n");
            file.write(line.as_bytes()).await?;
        }

        Ok(())
    }
}
