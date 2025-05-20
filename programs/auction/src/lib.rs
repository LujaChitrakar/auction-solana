use anchor_lang::prelude::*;

declare_id!("E6nE6seBRzfDrk1m96fKXKWxYe7JYWSpkFVMj5CLGeP6");

#[program]
pub mod auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
