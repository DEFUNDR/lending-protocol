use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface, InterfaceAccount, TokenAccount, TransferChecked};
use create::state::{Bank, User};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,    


    #[account(
        mut,
        seeds = [mint.key().as_ref()],
        bump,
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"treasury", bank.key().as_ref()],
        bump,
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump,
    )]
    pub user_account: InterfaceAccount<'info, User>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;

    let deposited_value : u64;
    if ctx.accounts.mint.to_account_info().key != user.usdc_address {
        deposited_value = user.deposited_usdc;
    } else {
        deposited_value = user.deposited_sol;
    }

    if amount > deposited_value {
        return Err(ErrorCode::InsufficientFunds.into()); //TODO: Custom error for insufficient funds
    }

    Ok(());
}