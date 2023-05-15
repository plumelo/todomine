use anyhow::Result;
use clap::Parser;
use sync::Sync;

mod issues;
mod sync;
mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Sync::parse();
    cli.sync().await?;
    Ok(())
}
