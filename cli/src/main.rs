mod config;

use clap::{Parser, Subcommand};
use config::Config;

#[derive(Parser)]
#[command(name = "divw")]
#[command(about = "DIVW Protocol CLI")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long)]
    rpc: Option<String>,
    
    #[arg(short, long)]
    keypair: Option<String>,
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    
    let rpc_url = cli.rpc.unwrap_or(config.rpc_url);
    
    match cli.command {
        Commands::Init => {
            println!("Initializing protocol on {}...", rpc_url);
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
    
    Ok(())
}
