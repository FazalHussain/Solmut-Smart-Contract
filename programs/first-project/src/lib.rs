use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

declare_id!("Am3MaWBXAj7exW93rDzJvB2TqTzeLA2XQ9gUaPHKAAT3");

#[program]
pub mod first_project {
    use super::*;

    pub fn buy_slmt(ctx: Context<BuySLMT>, sol_sent: u64) -> Result<()> {
        

        msg!("Presale received {} lamports", sol_sent);
        require!(sol_sent > 0, PresaleError::InsufficientPayment);

        let slmt_amount = sol_sent
            .checked_mul(10) // 10 = 10^1 (because SLMT has 1 decimal)
            .ok_or(PresaleError::Overflow)?
            .checked_div(1_900_000) // 0.0019 SOL in lamports
            .ok_or(PresaleError::Overflow)?;
        let decimals = ctx.accounts.mint.decimals;

        // Transfer SLMT from Phantom wallet to buyer
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token_interface::transfer_checked(cpi_ctx, slmt_amount, decimals)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuySLMT<'info> {
    #[account(mut)]
    pub sender: Signer<'info>, // Phantom wallet sending SLMT

    #[account(mut)]
    pub presale: SystemAccount<'info>, // Receives SOL

    #[account(
        mut,
        owner = token_program.key(),
        constraint = sender_token_account.owner == sender.key() @ PresaleError::InvalidVaultOwner
    )]
    pub sender_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        owner = token_program.key(),
        constraint = user_token_account.owner == buyer.key() @ PresaleError::InvalidUserTokenAccount
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub buyer: SystemAccount<'info>, // Receives SLMT

    pub mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[error_code]
pub enum PresaleError {
    #[msg("You must send SOL to buy SLMT.")]
    InsufficientPayment,

    #[msg("Overflow occurred during SLMT calculation.")]
    Overflow,

    #[msg("Sender token account is not owned by sender.")]
    InvalidVaultOwner,

    #[msg("User token account is not owned by the buyer.")]
    InvalidUserTokenAccount,
}
