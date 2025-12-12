use crate::error::DivwError;
use crate::events::DiveAborted;
use crate::state::{DiveState, DiveStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AbortDive<'info> {
    #[account(
        mut,
        seeds = [b"dive", diver.key().as_ref()],
        bump = dive_state.bump,
        constraint = dive_state.diver == diver.key() @ DivwError::Unauthorized
    )]
    pub dive_state: Account<'info, DiveState>,

    pub diver: Signer<'info>,
}

pub fn handler(ctx: Context<AbortDive>) -> Result<()> {
    let dive = &mut ctx.accounts.dive_state;

    require!(dive.is_hovering(), DivwError::InvalidDiveStatus);

    dive.status = DiveStatus::Aborted;

    emit!(DiveAborted {
        diver: dive.diver,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
