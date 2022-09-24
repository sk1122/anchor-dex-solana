use anchor_lang::{prelude::*};

declare_id!("BBf9Y9MctDrsip8ejGpfJhSXGacaYQEscesvAMVzKSsR");

#[program]
pub mod dex {
    use super::*;

    pub fn vote(ctx: Context<VoteContext>, yes: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;

        if yes {
            vote.yes += 1;
        } else {
            vote.no += 1;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VoteContext<'info> {
    #[account(
        init,
        payer = signer,
        space = 10240,
    )]
    pub vote: Account<'info, VoteAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct VoteAccount {
    pub yes: u64,
    pub no: u64
}

#[error_code]
pub enum ErrCode {
    #[msg("Authority Key not equal to Swap Authority Key")]
    AuthorityKeyNotEqualToSwapAuthorityKey
}