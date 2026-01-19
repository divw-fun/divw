mod config;
mod error;

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

    #[arg(short, long)]
    verbose: bool,
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

    if cli.verbose {
        println!("RPC: {}", cli.rpc.as_ref().unwrap_or(&config.rpc_url));
        println!("Keypair: {:?}", config.keypair_path);
    }

    config.validate()?;

    match cli.command {
        Commands::Init => {
            println!("Initializing protocol...");
        }
        Commands::Dive { depth, wire } => {
            if !(1..=10).contains(&depth) {
                return Err(error::CliError::InvalidDepth.into());
            }
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
