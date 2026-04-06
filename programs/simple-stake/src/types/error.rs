use anchor_lang::prelude::*;

#[error_code]
pub enum StakeProgramError {
  #[msg("Invalid input provided")]
  InvalidInput,
  #[msg("Account passed is already initialized")]
  AlreadyInitialized,
  #[msg("insufficient balance in your account")]
  InsufficientBalance,
  #[msg("Not authorized to perform this instruction")]
  Unauthorized,
}