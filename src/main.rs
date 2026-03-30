use clap::{Parser, Subcommand};

mod init;
use init::create_files::init_repo;

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
            let _ =init_repo();
        }
        Commands::Add { file } => {
            println!("Adding {}", file);
        }
        Commands::Commit { message } => {
            println!("Commit: {}", message);
        }
    }
}

