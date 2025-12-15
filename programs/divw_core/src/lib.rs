use anchor_lang::prelude::*;

mod constants;
mod error;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("DivWxJ4z2g9WpL7kM8tV5jN3rB6qY1hF9sX4cE2vA3m");

#[program]
pub mod divw_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        instructions::initialize::handler(ctx, bump)
    }

    pub fn create_dive(ctx: Context<CreateDive>, depth: u8, wire_length: u64) -> Result<()> {
        instructions::create_dive::handler(ctx, depth, wire_length)
    }

    pub fn spool_up(ctx: Context<SpoolUp>, priority: bool) -> Result<()> {
        instructions::spool_up::handler(ctx, priority)
    }

    pub fn abort_dive(ctx: Context<AbortDive>) -> Result<()> {
        instructions::abort_dive::handler(ctx)
    }

    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        instructions::admin::handler(ctx)
    }

    pub fn transfer_authority(ctx: Context<TransferAuthority>) -> Result<()> {
        instructions::transfer_authority::handler(ctx)
    }
}
