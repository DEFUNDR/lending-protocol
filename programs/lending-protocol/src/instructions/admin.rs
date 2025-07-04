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