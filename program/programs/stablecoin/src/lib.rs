use anchor_lang::prelude::*;

declare_id!("7icwNCcchMzMjQa3Z2mjSkiFz6GCJjADanYw2HBowmo4");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
