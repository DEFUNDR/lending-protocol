use anchor_lang::prelude::*;

declare_id!("5XkQCsgqwCfs2NWyj8fABGAsY6EQFxWhQHsJieQASoVj");

#[program]
pub mod lending_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
