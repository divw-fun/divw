use anchor_lang::prelude::*;
use divw_math::SafeMath;
use crate::state::{ProtocolState, DiveState, DiveStatus};
use crate::events::SpoolCompleted;
use crate::error::DivwError;
use crate::constants::PRIORITY_MULTIPLIER;

#[derive(Accounts)]
pub struct SpoolUp<'info> {
    #[account(mut, seeds = [b"protocol"], bump = protocol_state.bump)]
    pub protocol_state: Account<'info, ProtocolState>,
    
    #[account(
        mut,
        seeds = [b"dive", diver.key().as_ref()],
        bump = dive_state.bump,
        constraint = dive_state.diver == diver.key() @ DivwError::Unauthorized
    )]
    pub dive_state: Account<'info, DiveState>,
    
    pub diver: Signer<'info>,
}

pub fn handler(ctx: Context<SpoolUp>, priority: bool) -> Result<()> {
    let dive = &mut ctx.accounts.dive_state;
    
    require!(dive.is_hovering(), DivwError::InvalidDiveStatus);
    
    let wire_multiplier = if priority { PRIORITY_MULTIPLIER } else { 1 };
    let effective_wire = dive.wire_length.safe_mul(wire_multiplier)?;
    
    dive.status = DiveStatus::Surfaced;
    dive.executed_at = Clock::get()?.unix_timestamp;
    
    let protocol = &mut ctx.accounts.protocol_state;
    protocol.successful_spools = protocol.successful_spools.safe_add(1)?;
    
    emit!(SpoolCompleted {
        diver: dive.diver,
        effective_wire,
        priority,
        timestamp: dive.executed_at,
    });
    
    Ok(())
}
