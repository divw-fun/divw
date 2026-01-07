use crate::error::DivwError;
use crate::state::ProtocolState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(
        mut,
        seeds = [b"protocol"],
        bump = protocol_state.bump,
        constraint = protocol_state.authority == authority.key() @ DivwError::Unauthorized
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<TogglePause>) -> Result<()> {
    let state = &mut ctx.accounts.protocol_state;
    state.is_active = !state.is_active;
    Ok(())
}
