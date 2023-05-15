use crate::issues::Issues;
use crate::tasks::Tasks;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Synchronize Redmine issues with todo.txt files.", long_about = None)]
pub struct Sync {
    /// The todo.txt file to use
    #[arg(short, long, env = "TODOMINE_FILE")]
    file: String,
    /// The Redmine base url
    #[arg(short, long, env = "TODOMINE_API")]
    url: String,
    /// The Redmine api key
    #[arg(short, long, env = "TODOMINE_KEY")]
    key: String,
    /// The Redmine project
    #[arg(short, long, env = "TODOMINE_PROJECT")]
    project: Option<String>,
}

impl Sync {
    pub async fn sync(self) -> Result<()> {
        let tasks = Tasks::new(self.file).read().await?;
        let issues = Issues::new(self.url, self.key, self.project).get().await?;
        tasks.sync(issues).write().await?;
        Ok(())
    }
}
