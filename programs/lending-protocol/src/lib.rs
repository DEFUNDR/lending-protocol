use anchor_lang::prelude::*;
use instructions::*;

mod state;

declare_id!("5XkQCsgqwCfs2NWyj8fABGAsY6EQFxWhQHsJieQASoVj");

#[program]
pub mod lending_protocol {
    use super::*;

    pub fn init_bank(ctx: Context<Initialize>, liquidation_threshold: u64, max_ltv: u64) -> Result<()> {
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }
}


