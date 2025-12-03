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
