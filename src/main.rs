use clap::{Parser, Subcommand};

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
            println!("Initializing repo...");
        }
        Commands::Add { file } => {
            println!("Adding {}", file);
        }
        Commands::Commit { message } => {
            println!("Commit: {}", message);
        }
    }
}

