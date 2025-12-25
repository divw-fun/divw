use std::path::PathBuf;
use anyhow::Result;

pub struct Config {
    pub rpc_url: String,
    pub keypair_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let keypair_path = PathBuf::from(format!("{}/.config/solana/id.json", home));
        
        Ok(Self {
            rpc_url: "https://api.devnet.solana.com".to_string(),
            keypair_path,
        })
    }
}
