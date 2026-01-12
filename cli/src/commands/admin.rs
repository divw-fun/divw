use clap::Subcommand;

#[derive(Subcommand)]
pub enum AdminCommands {
    Pause,
    Unpause,
    TransferAuthority {
        #[arg(long)]
        new_authority: String,
    },
    Status,
}

pub fn handle_admin(cmd: AdminCommands) {
    match cmd {
        AdminCommands::Pause => {
            println!("Pausing protocol...");
        }
        AdminCommands::Unpause => {
            println!("Unpausing protocol...");
        }
        AdminCommands::TransferAuthority { new_authority } => {
            println!("Transferring authority to {}...", new_authority);
        }
        AdminCommands::Status => {
            println!("Fetching admin status...");
        }
    }
}
