use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dex {
    use super::*;

    pub fn create_liquidity_pools(ctx: Context<CreateLiquidityPool>, fees: Fees) -> Result<()> {
        if ctx.accounts.liquidity_pool.is_initialized {
            
        }

        let (swap_authority, bump_seed) = Pubkey::find_program_address(
            &[&ctx.accounts.liquidity_pool.to_account_info().key.to_bytes()],
            ctx.program_id
        );

        let seeds = &[
            &ctx.accounts.liquidity_pool.to_account_info().key.to_bytes(),
            &[bump_seed][..],
        ];

        if *ctx.accounts.authority.key != swap_authority {
            
        }

        let liquidity_pool = &mut ctx.accounts.liquidity_pool;

        liquidity_pool.fees = fees;
        liquidity_pool.is_initialized = true;
        liquidity_pool.bump_seed = bump_seed;
        liquidity_pool.token_a_account = *ctx.accounts.token_a.to_account_info().key;
        liquidity_pool.token_b_account = *ctx.accounts.token_b.to_account_info().key;
        liquidity_pool.token_a_mint = ctx.accounts.token_a.mint;
        liquidity_pool.token_b_mint = ctx.accounts.token_b.mint;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateLiquidityPool<'info> {
    /// CHECK: safe
    pub authority: AccountInfo<'info>,
    #[account(
        init, 
        payer = signer, 
        space = 8 + 1 + 8 + 8, 
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub token_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>,
    /// CHECK: safe
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Fees {
    pub percentage: u64
}

#[account]
#[derive(Default)]
pub struct LiquidityPool {
    pub fees: Fees,
    pub is_initialized: bool,
    pub bump_seed: u8,
    pub token_program_id: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub fee_account: Pubkey,
}