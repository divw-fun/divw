use std::path::PathBuf;
use anyhow::Result;

pub const DEFAULT_RPC: &str = "https://api.devnet.solana.com";
pub const DEFAULT_MAINNET_RPC: &str = "https://api.mainnet-beta.solana.com";

pub struct Config {
    pub rpc_url: String,
    pub keypair_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let keypair_path = PathBuf::from(format!("{}/.config/solana/id.json", home));
        
        Ok(Self {
            rpc_url: DEFAULT_RPC.to_string(),
            keypair_path,
        })
    }
    
    pub fn with_rpc(mut self, rpc: String) -> Self {
        self.rpc_url = rpc;
        self
    }
}
