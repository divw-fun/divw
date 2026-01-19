use anyhow::Result;
use std::path::PathBuf;

pub const DEFAULT_RPC: &str = "https://api.devnet.solana.com";

pub struct Config {
    pub rpc_url: String,
    pub keypair_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());

        let keypair_path = PathBuf::from(format!("{}/.config/solana/id.json", home));

        Ok(Self {
            rpc_url: DEFAULT_RPC.to_string(),
            keypair_path,
        })
    }

    pub fn keypair_exists(&self) -> bool {
        self.keypair_path.exists()
    }

    pub fn validate(&self) -> Result<()> {
        if !self.keypair_exists() {
            anyhow::bail!(
                "Keypair not found at {:?}. Run 'solana-keygen new' to create one.",
                self.keypair_path
            );
        }
        Ok(())
    }
}
