use crate::constants::{BASE_WIRE_LENGTH, MAX_DEPTH};
use crate::error::DivwError;
use crate::events::DiveCreated;
use crate::state::{DiveState, DiveStatus, ProtocolState};
use anchor_lang::prelude::*;
use divw_math::SafeMath;

#[derive(Accounts)]
pub struct CreateDive<'info> {
    #[account(mut, seeds = [b"protocol"], bump = protocol_state.bump)]
    pub protocol_state: Account<'info, ProtocolState>,

    #[account(
        init,
        seeds = [b"dive", diver.key().as_ref()],
        bump,
        payer = diver,
        space = 8 + DiveState::LEN
    )]
    pub dive_state: Account<'info, DiveState>,

    #[account(mut)]
    pub diver: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateDive>, depth: u8, wire_length: u64) -> Result<()> {
    require!(depth > 0 && depth <= MAX_DEPTH, DivwError::InvalidDepth);
    require!(wire_length >= BASE_WIRE_LENGTH, DivwError::InsufficientWire);

    let dive = &mut ctx.accounts.dive_state;
    dive.diver = ctx.accounts.diver.key();
    dive.depth = depth;
    dive.wire_length = wire_length;
    dive.status = DiveStatus::Hovering;
    dive.created_at = Clock::get()?.unix_timestamp;
    dive.executed_at = 0;
    dive.bump = ctx.bumps.dive_state;

    let protocol = &mut ctx.accounts.protocol_state;
    protocol.total_dives = protocol.total_dives.safe_add(1)?;

    emit!(DiveCreated {
        diver: dive.diver,
        depth,
        wire_length,
        timestamp: dive.created_at,
    });

    Ok(())
}
