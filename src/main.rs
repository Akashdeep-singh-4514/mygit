use clap::{Parser, Subcommand};

mod add;
mod index;
mod init;
mod shas;
use add::add_file::add;
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
            let _ = init_repo();
        }
        Commands::Add { file } => {
            if let Err(e) = add(&file) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Commit { message } => {
            println!("Commit: {}", message);
        }
    }
}
