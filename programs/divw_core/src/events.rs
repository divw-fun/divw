use anchor_lang::prelude::*;

#[event]
pub struct ProtocolInitialized {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DiveCreated {
    pub diver: Pubkey,
    pub depth: u8,
    pub wire_length: u64,
    pub timestamp: i64,
}

#[event]
pub struct SpoolCompleted {
    pub diver: Pubkey,
    pub effective_wire: u64,
    pub priority: bool,
    pub timestamp: i64,
}

#[event]
pub struct DiveAborted {
    pub diver: Pubkey,
    pub timestamp: i64,
}
