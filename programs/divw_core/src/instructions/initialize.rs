use crate::events::ProtocolInitialized;
use crate::state::ProtocolState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"protocol"],
        bump,
        payer = authority,
        space = 8 + ProtocolState::LEN
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, bump: u8) -> Result<()> {
    let state = &mut ctx.accounts.protocol_state;
    state.authority = ctx.accounts.authority.key();
    state.bump = bump;
    state.total_dives = 0;
    state.successful_spools = 0;
    state.is_active = true;

    emit!(ProtocolInitialized {
        authority: state.authority,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
