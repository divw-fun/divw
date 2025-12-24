use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "divw")]
#[command(about = "DIVW Protocol CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "https://api.devnet.solana.com")]
    rpc: String,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Dive {
        #[arg(short, long)]
        depth: u8,
        #[arg(short, long, default_value = "100000")]
        wire: u64,
    },
    Spool {
        #[arg(short, long)]
        priority: bool,
    },
    Abort,
    Status,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init => {
            println!("Initializing protocol...");
        }
        Commands::Dive { depth, wire } => {
            println!("Creating dive: depth={}, wire={}", depth, wire);
        }
        Commands::Spool { priority } => {
            println!("Spooling up (priority={})...", priority);
        }
        Commands::Abort => {
            println!("Aborting dive...");
        }
        Commands::Status => {
            println!("Fetching status...");
        }
    }
}
