use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, InterfaceAccount, TokenAccount};
use create::state::Bank;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [mint.key().as_ref()],
        bump,
    )]

    pub bank: Account<'info, Bank>,

    #[account(
        init,
        token::mint = mint,
        token::authority = bank_token_account,
        payer = signer,
        seeds = [b"treasury", bank.key().as_ref()],
        bump,
    )]

    pub bank_token_account: InterfaceAccount<'info, anchor_spl::token::TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE, // Assuming User is defined elsewhere
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>, // Assuming User is defined elsewhere

    pub system_program: Program<'info, System>,
}

pub fn process_init_bank(ctx: Context<InitBank>, liquidation_threshold : u64, max_ltv : u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    bank.mint_address = ctx.accounts.mint.key();
    bank.authority = ctx.accounts.signer.key();
    bank.liquidation_threshold = liquidation_threshold;
    bank.max_ltv = max_ltv;

    Ok(())
}

pub fn process_init_user(ctx: Context<InitUser>, usdc_address : Pubkey) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.owner = ctx.accounts.signer.key();
    user_account.usdc_address = usdc_address;

    Ok(())
}