use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Custom error message")]
    FreezePeriodNotExpired,
    
    #[msg("max stake amount reached")]
    MaxStakeReached
}
