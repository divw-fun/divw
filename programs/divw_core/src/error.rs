use anchor_lang::prelude::*;

#[error_code]
pub enum DivwError {
    #[msg("Depth must be between 1 and 10")]
    InvalidDepth,

    #[msg("Wire length insufficient for this operation")]
    InsufficientWire,

    #[msg("Invalid dive status for this operation")]
    InvalidDiveStatus,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Protocol is currently paused")]
    ProtocolPaused,

    #[msg("Math overflow")]
    MathOverflow,
}
