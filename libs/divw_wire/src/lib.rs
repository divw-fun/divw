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

/// Calculate wire tension based on network conditions
pub fn calculate_tension(pending_txs: u32, gas_price: u64, slot_latency: u32) -> u8 {
    let load_factor = (pending_txs as u64).saturating_mul(gas_price);
    let tension = load_factor.saturating_add(slot_latency as u64);
    std::cmp::min(tension / 1000, 100) as u8
}

/// Check if wire should trigger auto-spool based on current tension
pub fn should_auto_spool(config: &WireConfig, current_tension: u8) -> bool {
    current_tension <= config.tension_threshold
}
