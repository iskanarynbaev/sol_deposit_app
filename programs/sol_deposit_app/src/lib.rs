use anchor_lang::prelude::*;

declare_id!("BP3wxgiqMzh15tiaRAsV8MSr73K6sHnd1FgbjxtjDo3L");

#[program]
pub mod deposit_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_account.balance = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        ctx.accounts.vault_account.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(
            ctx.accounts.vault_account.balance >= amount,
            ErrorCode::InsufficientFunds
        );

        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

        ctx.accounts.vault_account.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"vault_account", user.key().as_ref()], bump)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is safe, only used to receive SOL
    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"vault_account", user.key().as_ref()], bump)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Receives SOL
    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault_account", user.key().as_ref()], bump)]
    pub vault_account: Account<'info, VaultAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: Sends SOL
    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: AccountInfo<'info>,
}

#[account]
pub struct VaultAccount {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in vault.")]
    InsufficientFunds,
}