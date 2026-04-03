use clap::{Parser, Subcommand};

use mygit::commands::{add, commit, init};

#[derive(Parser)]
#[command(name = "mygit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { file: String },
    Commit { message: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            let _ = init::init_repo();
        }
        Commands::Add { file } => {
            if let Err(e) = add::add(&file) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Commit { message } => {
            commit::commit(&message);
        }
    }
}
