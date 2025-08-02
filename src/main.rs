mod main_fs;
mod web;

use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(name = "mosaic")]
#[command(about = "The package manager for Mountain Sakura projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, clap::Subcommand)]
#[derive(Clone)]
enum Commands {
    Download {
        name: String
    }
}

#[tokio::main]
async fn main() {
    let parse = Cli::parse();

    match parse.command {
        Commands::Download { name } => {
            main_fs::download(name).await.unwrap()
        }
    }
}
