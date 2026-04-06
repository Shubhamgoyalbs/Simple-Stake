pub mod types;
pub mod instructions;

use types::*;
use instructions::*;

use anchor_lang::prelude::*;

declare_id!("7WcSt7ZKnTMpVCHwcNcGw4C1pgsSfphPit9amPdd5htK");

#[program]
pub mod simple_stake {
  use super::*;
  
  pub fn user_initialize(ctx: Context<UserInitialize>) -> Result<()> {
    user_init_processor(ctx)
  }
  
  pub fn pool_initialize(
    ctx: Context<PoolInitialize>,
    min_lock_duration: i64,
    min_stake_required: u64,
    reward_rate: u64,
    is_active: bool,
    reward_reserve: u64
  ) -> Result<()> {
    pool_init_processor(ctx, min_lock_duration, min_stake_required, reward_rate, reward_reserve, is_active)
  }
}
