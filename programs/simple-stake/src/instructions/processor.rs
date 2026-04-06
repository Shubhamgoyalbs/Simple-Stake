use crate::types::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

pub fn user_init_processor(ctx: Context<UserInitialize>) -> Result<()> {
  let data = &mut ctx.accounts.user_pda;
  data.authority = *ctx.accounts.signer.key;
  data.total_rewards_earned = 0;
  data.available_balance = 0;
  data.bump = ctx.bumps.user_pda;
  Ok(())
}

pub fn pool_init_processor(
  ctx: Context<PoolInitialize>,
  min_lock_duration: i64,
  min_stake_required: u64,
  reward_rate: u64,
  reward_reserve: u64,
  is_active: bool,
) -> Result<()> {
  
  let cpi_ctx = CpiContext::new(
    ctx.accounts.system_program.to_account_info(),
    Transfer {
      from: ctx.accounts.signer.to_account_info(),
      to: ctx.accounts.pool_pda.to_account_info(),
    },
  );
  
  let data = &mut ctx.accounts.pool_pda;
  data.authority = *ctx.accounts.signer.key;
  data.min_lock_duration = min_lock_duration;
  data.min_stake_required = min_stake_required;
  data.reward_rate = reward_rate;
  data.is_active = is_active;
  data.total_staked = 0;
  data.bump = ctx.bumps.pool_pda;
  
  transfer(cpi_ctx, reward_reserve)?;
  
  data.reward_reserve = reward_reserve;
  Ok(())
}

pub fn claim_balance(ctx: Context<ClaimBalance>) -> Result<()> {
  let accounts = ctx.accounts;
  if accounts.user_pda.available_balance == 0 {
    return err!(StakeProgramError::InsufficientBalance);
  }
  let seeds = &[
    b"user",
    accounts.user_pda.authority.as_ref(),
    &[accounts.user_pda.bump],
  ];
  let signer_seeds = &[&seeds[..]];
  
  let cpi_ctx = CpiContext::new_with_signer(
    accounts.system_program.to_account_info(),
    Transfer {
      from: accounts.user_pda.to_account_info(),
      to: accounts.signer.to_account_info(),
    },
    signer_seeds,
  );
  transfer(cpi_ctx, accounts.user_pda.available_balance)?;
  
  accounts.user_pda.available_balance = 0;
  Ok(())
}

pub fn stake_asset(ctx: Context<StakeAsset>, amount: u64, lock_duration: i64) -> Result<()> {
  let accounts = ctx.accounts;
  if accounts.stake_pda.is_staked {
    return err!(StakeProgramError::InvalidInput);
  }
  if amount < accounts.pool_pda.min_stake_required {
    return err!(StakeProgramError::InvalidInput);
  }
  if lock_duration < accounts.pool_pda.min_lock_duration {
    return err!(StakeProgramError::InvalidInput);
  }
  let cpi_ctx = CpiContext::new(
    accounts.system_program.to_account_info(),
    Transfer {
      from: accounts.signer.to_account_info(),
      to: accounts.pool_pda.to_account_info(),      // pool PDA holds the asset
    },
  );
  
  transfer(cpi_ctx, amount)?;
  accounts.stake_pda.user = *accounts.signer.key;
  accounts.stake_pda.pool = accounts.pool_pda.key();
  accounts.stake_pda.is_staked = true;
  accounts.stake_pda.staked_at = Clock::get()?.unix_timestamp;
  accounts.stake_pda.lock_duration = lock_duration;
  accounts.stake_pda.amount = amount;
  accounts.stake_pda.bump = ctx.bumps.stake_pda;
  
  accounts.pool_pda.total_staked += amount;
  Ok(())
}
//unstake (all money came to its pda),
// fund pool always reach min lock duration * (total staked + min stake) * reward rate,
// claim reward resevve only if staking is closed and no staker remains here,
// note can check if avialable reward reserve then add else close accepting stake for that pool

