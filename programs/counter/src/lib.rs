use anchor_lang::prelude::*;

declare_id!("AhPUzT1vHWwBLBmdbtt91rkcsoyTHaZErJzFaYNpJeQk");

#[program]
mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, start: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = start;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        let old_count = counter.count;
        counter.count = old_count + 1;
        msg!("+1! before={}, after={}", old_count, counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 48)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey, // 8bytes
    pub count: u64, // 8bytes
}
