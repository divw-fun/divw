use anchor_lang::prelude::*;

pub trait SafeMath {
    fn safe_add(&self, v: u64) -> Result<u64>;
    fn safe_sub(&self, v: u64) -> Result<u64>;
    fn safe_mul(&self, v: u64) -> Result<u64>;
    fn safe_div(&self, v: u64) -> Result<u64>;
}

impl SafeMath for u64 {
    #[inline]
    fn safe_add(&self, v: u64) -> Result<u64> {
        self.checked_add(v)
            .ok_or_else(|| error!(MathError::Overflow))
    }

    #[inline]
    fn safe_sub(&self, v: u64) -> Result<u64> {
        self.checked_sub(v)
            .ok_or_else(|| error!(MathError::Underflow))
    }

    #[inline]
    fn safe_mul(&self, v: u64) -> Result<u64> {
        self.checked_mul(v)
            .ok_or_else(|| error!(MathError::Overflow))
    }

    #[inline]
    fn safe_div(&self, v: u64) -> Result<u64> {
        if v == 0 {
            return Err(error!(MathError::DivisionByZero));
        }
        self.checked_div(v)
            .ok_or_else(|| error!(MathError::DivisionByZero))
    }
}

#[inline]
pub fn calculate_wire_extension(base: u64, token_amount: u64) -> Result<u64> {
    if token_amount == 0 {
        return Ok(base);
    }
    let bonus = base.safe_mul(token_amount)?.safe_div(100)?;
    base.safe_add(bonus)
}

#[inline]
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
    }

    #[test]
    fn test_safe_div_zero() {
        assert!(5u64.safe_div(0).is_err());
    }

    #[test]
    fn test_wire_extension_zero_tokens() {
        assert_eq!(calculate_wire_extension(100_000, 0).unwrap(), 100_000);
    }

    #[test]
    fn test_wire_extension_with_tokens() {
        assert_eq!(calculate_wire_extension(100_000, 50).unwrap(), 150_000);
    }
}
