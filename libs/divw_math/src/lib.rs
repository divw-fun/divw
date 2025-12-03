use anchor_lang::prelude::*;

pub trait SafeMath {
    fn safe_add(&self, v: u64) -> Result<u64>;
    fn safe_mul(&self, v: u64) -> Result<u64>;
}
