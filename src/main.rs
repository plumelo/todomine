use clap::{Args, Parser, Subcommand};
use create::create;

mod create;

#[derive(Parser)]
#[command(name = "todomine")]
#[command(author = "Iulian M. <iulian.meghea@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Synchronize Redmine issues with todo.txt files.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new todo.txt file from Redmine issues.
    Create(CreateArgs),
}

#[derive(Args)]
struct RedmineArgs {
    project: String,
}

#[derive(Args)]
struct CreateArgs {
    name: String,
    url: String,
    key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(name) => {
            println!("'myapp create' was used, name is: {:?}", name.name);
            create();
        }
    }
    println!("Hello, world!");
}
