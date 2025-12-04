use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum WireState {
    Slack,
    Taut,
    Spooling,
    Retracted,
    Snapped,
}

impl Default for WireState {
    fn default() -> Self {
        WireState::Slack
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WireConfig {
    pub max_length: u64,
    pub tension_threshold: u8,
    pub priority: u8,
    pub timeout_slots: u64,
}

impl Default for WireConfig {
    fn default() -> Self {
        Self {
            max_length: 100_000,
            tension_threshold: 50,
            priority: 0,
            timeout_slots: 150,
        }
    }
}
