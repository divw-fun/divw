use anchor_lang::prelude::*;

pub trait SafeMath {
    fn safe_add(&self, v: u64) -> Result<u64>;
    fn safe_sub(&self, v: u64) -> Result<u64>;
    fn safe_mul(&self, v: u64) -> Result<u64>;
    fn safe_div(&self, v: u64) -> Result<u64>;
}

impl SafeMath for u64 {
    fn safe_add(&self, v: u64) -> Result<u64> {
        self.checked_add(v).ok_or_else(|| error!(MathError::Overflow))
    }

    fn safe_sub(&self, v: u64) -> Result<u64> {
        self.checked_sub(v).ok_or_else(|| error!(MathError::Underflow))
    }

    fn safe_mul(&self, v: u64) -> Result<u64> {
        self.checked_mul(v).ok_or_else(|| error!(MathError::Overflow))
    }

    fn safe_div(&self, v: u64) -> Result<u64> {
        self.checked_div(v).ok_or_else(|| error!(MathError::DivisionByZero))
    }
}

/// Calculate wire extension based on token holdings
/// Each token grants 1% additional wire length
pub fn calculate_wire_extension(base: u64, token_amount: u64) -> Result<u64> {
    let bonus = base.safe_mul(token_amount)?.safe_div(100)?;
    base.safe_add(bonus)
}

/// Calculate priority score for transaction ordering
pub fn calculate_priority_score(depth: u8, wire_length: u64, is_holder: bool) -> u64 {
    let base_score = (depth as u64).saturating_mul(wire_length);
    if is_holder {
        base_score.saturating_mul(2)
    } else {
        base_score
    }
}

#[error_code]
pub enum MathError {
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Division by zero")]
    DivisionByZero,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_add() {
        assert_eq!(5u64.safe_add(3).unwrap(), 8);
        assert_eq!(0u64.safe_add(0).unwrap(), 0);
    }

    #[test]
    fn test_safe_mul() {
        assert_eq!(5u64.safe_mul(3).unwrap(), 15);
        assert_eq!(100u64.safe_mul(0).unwrap(), 0);
    }

    #[test]
    fn test_overflow() {
        assert!(u64::MAX.safe_add(1).is_err());
        assert!(u64::MAX.safe_mul(2).is_err());
    }

    #[test]
    fn test_wire_extension() {
        let result = calculate_wire_extension(100_000, 50).unwrap();
        assert_eq!(result, 150_000);
    }

    #[test]
    fn test_priority_score() {
        let score = calculate_priority_score(5, 1000, false);
        assert_eq!(score, 5000);
        
        let holder_score = calculate_priority_score(5, 1000, true);
        assert_eq!(holder_score, 10000);
    }
}
