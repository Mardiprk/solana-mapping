use anchor_lang::prelude::*;

declare_id!("8zy2dNPzUPX3kiqRAbBF4YZV3gvzoijmHbvyByM77rrb");

#[program]
pub mod solana_mapping {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
