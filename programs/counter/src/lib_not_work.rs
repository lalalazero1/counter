use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use std::ops::DerefMut;

declare_id!("3KYfpHFZZNofUQ3aLx6ZG9SZeCwXGWihpHrS3361AiCW");

// 智能合约的核心功能
#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!(
            "This is Counter Program, Greetings from ID: {:?}",
            ctx.program_id
        );

        let counter = ctx.accounts.counter.deref_mut();
        let bump = ctx.bumps.counter;

        *counter = Counter {
            authority: *ctx.accounts.authority.key,
            count: 0,
            bump,
        };

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.counter.authority,
            ErrorCode::Unauthorized
        );

        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

// 用 Initialize 结构体来明确哪些账户可以参与到初始化的过程中
// <'info> 标记结构体中所有引用（例如账户数据）的生命周期，确保这些引用在整个 Initialize 结构体的生命周期内都是有效的。这意味着，只要 Initialize 结构体存在，其中的账户数据就可以安全地被访问和使用
#[derive(Accounts)]
pub struct Initialize<'info> {
    // 需要创建新账户
    // 指定 payer
    // 指定账户预留空间
    // 需要派生地址 pda
    #[account(init,
       payer=authority,
       space= Counter::SIZE,
       seeds = [b"counter"],
       bump
    )]
    counter: Account<'info, Counter>,
    // Signer 即签名者， 表示有权限执行初始化的账户
    // mut 修饰表示这个账户不是固定不变的
    #[account(mut)]
    authority: Signer<'info>, // 32 bytes，258 bits
    pub system_program: Program<'info, System>, // 和 solana 交互的桥梁
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut,
       seeds = [b"counter"],
       bump = counter.bump
    )]
    counter: Account<'info, Counter>,
    authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey, // 32 bytes, 256 bits
    pub count: u64, // 8 bytes, 64 bits
    pub bump: u8,   // 1 bytes, 8 bits
}

impl Counter {
    // Counter 所需要的存储空间
    pub const SIZE: usize = 8 + 32 + 8 + 1; // 最开始的8表示8bytes的账户类型标识符
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("The requested operation is not permitted")]
    OperationNotAllowed,

    #[msg("The provided input is invalid")]
    InvalidInput,

    #[msg("Insufficient funds to perform the operation")]
    InsuffcientFunds,
}
