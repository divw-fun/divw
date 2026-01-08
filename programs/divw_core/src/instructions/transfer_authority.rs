use crate::error::DivwError;
use crate::state::ProtocolState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferAuthority<'info> {
    #[account(
        mut,
        seeds = [b"protocol"],
        bump = protocol_state.bump,
        constraint = protocol_state.authority == authority.key() @ DivwError::Unauthorized
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    pub authority: Signer<'info>,

    /// CHECK: New authority, validated by caller
    pub new_authority: AccountInfo<'info>,
}

pub fn handler(ctx: Context<TransferAuthority>) -> Result<()> {
    let state = &mut ctx.accounts.protocol_state;
    state.authority = ctx.accounts.new_authority.key();
    Ok(())
}
