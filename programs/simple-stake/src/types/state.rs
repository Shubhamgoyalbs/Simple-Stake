use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UserInitialize<'info> {
  #[account(
    init,
    payer = signer,
    space = 8 + User::INIT_SPACE,
    seeds = [
      b"user",
      signer.key().as_ref()
    ],
    bump
  )]
  pub user_pda: Account<'info, User>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct PoolInitialize<'info> {
  #[account(
    init,
    payer = signer,
    space = 8 + Pool::INIT_SPACE,
    seeds = [
      b"pool",
      signer.key().as_ref()
    ],
    bump
  )]
  pub pool_pda: Account<'info, Pool>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ClaimBalance<'info> {
  #[account(
    mut,
    seeds = [
      "user".as_ref(),
      signer.key().as_ref()
    ],
    bump = user_pda.bump
  )]
  pub user_pda: Account<'info, User>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UnStakeAsset<'info> {
  #[account(
    mut,
    seeds = [
      b"stake",
      signer.key().as_ref(),
      pool_pda.key().as_ref(),
    ],
    bump = stake_pda.bump,
    close = user_pda
  )]
  pub stake_pda: Account<'info, StakePosition>,
  
  #[account(
    mut,
    seeds = [
      "user".as_ref(),
      signer.key().as_ref()
    ],
    bump = user_pda.bump
  )]
  pub user_pda: Account<'info, User>,
  
  #[account(
    mut,
    seeds = [b"pool", pool_pda.authority.as_ref()],
    bump = pool_pda.bump,
  )]
  pub pool_pda: Account<'info, Pool>,
  
  #[account(mut)]
  pub signer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64, lock_duration: i64)]
pub struct StakeAsset<'info> {
  #[account(
    init,
    payer = signer,
    space = 8 + StakePosition::INIT_SPACE,
    seeds = [
      b"stake",
      signer.key().as_ref(),
      pool_pda.key().as_ref(),
    ],
    bump
  )]
  pub stake_pda: Account<'info, StakePosition>,
  
  #[account(
    mut,
    seeds = [b"pool", pool_pda.authority.as_ref()],
    bump = pool_pda.bump,
  )]
  pub pool_pda: Account<'info, Pool>,
  
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct FundPool<'info> {
  #[account(
    mut,
    seeds = [b"pool", pool_pda.authority.as_ref()],
    bump = pool_pda.bump,
  )]
  pub pool_pda: Account<'info, Pool>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ClaimRewardReserve<'info> {
  #[account(
    mut,
    seeds = [b"pool", pool_pda.authority.as_ref()],
    bump = pool_pda.bump,
  )]
  pub pool_pda: Account<'info, Pool>,
  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct User {
  pub authority: Pubkey,            // wallet owner
  
  pub total_rewards_earned: u128,   // total rewards earned over time
  pub available_balance: u64,      // withdrawable balance
  
  pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct StakePosition {
  pub user: Pubkey,          // reference to User PDA
  pub pool: Pubkey,          // pool in which user staked
  
  pub amount: u64,          // lamports staked
  pub staked_at: i64,        // unix timestamp
  pub lock_duration: i64,    // duration in seconds
  
  pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
  pub authority: Pubkey,          // pool creator/admin
  
  pub reward_rate: u64,          // reward per second (or define clearly), 1e9 precision (fraction)
  pub min_stake_required: u64,   // minimum stake required to get rewards
  
  pub min_lock_duration: i64,     // minimum lock time required
  pub total_staked: u64,         // total funds in pool
  pub reward_reserve: u64,
  
  pub is_active: bool,       // flag for is pool accepting stake
  pub bump: u8,
}