#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_state_default() {
        assert_eq!(WireState::default(), WireState::Slack);
    }

    #[test]
    fn test_wire_config_default() {
        let config = WireConfig::default();
        assert_eq!(config.max_length, 100_000);
        assert_eq!(config.tension_threshold, 50);
    }

    #[test]
    fn test_high_tension() {
        let tension = calculate_tension(1000, 1000, 1000);
        assert_eq!(tension, 100);
    }
}
