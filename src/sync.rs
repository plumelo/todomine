use crate::issues::ListIssues;
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
    #[arg(short, long, env = "TODOMINE_REDMINE_API")]
    url: String,
    /// The Redmine api key
    #[arg(short, long, env = "TODOMINE_REDMINE_KEY")]
    key: String,
}

impl Sync {
    pub async fn sync(self) -> Result<()> {
        let tasks = Tasks::new(self.file);
        let issue_list = ListIssues::new(self.url, self.key);
        let task_list = &mut issue_list.get().await?.into_tasks();
        tasks.sync(task_list).write().await?;
        Ok(())
    }
}
