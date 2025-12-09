use anchor_lang::prelude::*;

#[account]
pub struct ProtocolState {
    pub authority: Pubkey,
    pub bump: u8,
    pub total_dives: u64,
    pub successful_spools: u64,
    pub is_active: bool,
    pub _padding: [u8; 6],
}

impl ProtocolState {
    pub const LEN: usize = 32 + 1 + 8 + 8 + 1 + 6;

    pub fn is_authority(&self, key: &Pubkey) -> bool {
        self.authority == *key
    }
}
