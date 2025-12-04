use anchor_lang::prelude::*;

/// Wire connection state
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
