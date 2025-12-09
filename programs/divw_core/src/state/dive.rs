use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiveStatus {
    #[default]
    Hovering,
    Surfaced,
    Aborted,
}

#[account]
pub struct DiveState {
    pub diver: Pubkey,
    pub depth: u8,
    pub wire_length: u64,
    pub status: DiveStatus,
    pub created_at: i64,
    pub executed_at: i64,
    pub bump: u8,
    pub _padding: [u8; 4],
}

impl DiveState {
    pub const LEN: usize = 32 + 1 + 8 + 1 + 8 + 8 + 1 + 4;

    pub fn is_hovering(&self) -> bool {
        self.status == DiveStatus::Hovering
    }
}
