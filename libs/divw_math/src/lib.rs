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
}
