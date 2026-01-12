use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum WireState {
    #[default]
    Slack,
    Taut,
    Spooling,
    Retracted,
    Snapped,
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

pub fn calculate_tension(pending_txs: u32, gas_price: u64, slot_latency: u32) -> u8 {
    if pending_txs == 0 && slot_latency == 0 {
        return 0;
    }
    let load_factor = (pending_txs as u64).saturating_mul(gas_price);
    let tension = load_factor.saturating_add(slot_latency as u64);
    std::cmp::min(tension / 1000, 100) as u8
}

pub fn should_auto_spool(config: &WireConfig, current_tension: u8) -> bool {
    current_tension <= config.tension_threshold
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_tension() {
        assert_eq!(calculate_tension(0, 0, 0), 0);
    }

    #[test]
    fn test_auto_spool_threshold() {
        let config = WireConfig::default();
        assert!(should_auto_spool(&config, 50));
        assert!(!should_auto_spool(&config, 51));
    }
}
