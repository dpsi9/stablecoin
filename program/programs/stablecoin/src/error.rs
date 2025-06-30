use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Below minimum health factor")]
    BelowMinimumHealthFactor,
    #[msg("Above minimum health factor, Cannot liquidate Healthy account")]
    AboveMinimumHealthFactor,
    #[msg("Price should not be negative")]
    InvalidPrice,
}
