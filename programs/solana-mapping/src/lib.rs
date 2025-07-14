use anchor_lang::prelude::*;

declare_id!("8zy2dNPzUPX3kiqRAbBF4YZV3gvzoijmHbvyByM77rrb");

#[program]
pub mod simple_map {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.authority = ctx.accounts.authority.key();
        user_account.data = data;
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_value: u64) -> Result<()> {
        let account = &mut ctx.accounts.user_account;
        require_keys_eq!(
            account.authority,
            ctx.accounts.authority.key(),
            CustomError::UnAuthorized
        );
        account.data = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8,
        seeds = [b"user", authority.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub authority: Signer<'info>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub data: u64,
}

#[error_code]
enum CustomError {
    #[msg("you're not authorized")]
    UnAuthorized,
}