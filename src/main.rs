use anyhow::Result;
use clap::Parser;
use sync::Sync;

mod issues;
mod sync;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Sync::parse();
    cli.sync().await?;
    Ok(())
}
