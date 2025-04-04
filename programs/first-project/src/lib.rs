use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_spl::{
    token_interface::{
        self, Mint, TokenAccount, TokenInterface, TransferChecked
    }
};

declare_id!("Am3MaWBXAj7exW93rDzJvB2TqTzeLA2XQ9gUaPHKAAT3");

#[program]
pub mod first_project {
    use super::*;

    pub fn buy_slmt(ctx: Context<BuySLMT>, presale_lamports_before: u64, decimals: u8) -> Result<()> {
        let sol_sent = ctx
            .accounts
            .presale
            .lamports()
            .saturating_sub(presale_lamports_before);

        require!(sol_sent > 0, PresaleError::InsufficientPayment);

        let slmt_amount = sol_sent.checked_mul(20_000).ok_or(PresaleError::Overflow)?;

        let bump = ctx.bumps.vault_authority;
        let signer_seeds: &[&[u8]] = &[b"vault", &[bump]];
        let signer = &[signer_seeds];

        // Create CPI context following the TransferChecked struct pattern [(1)](https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens)
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.vault_token_account.to_account_info(), 
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        
        token_interface::transfer_checked(cpi_ctx, slmt_amount, decimals)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuySLMT<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [b"vault"],
        bump
    )]
    /// CHECK: PDA authority only
    pub vault_authority: UncheckedAccount<'info>,

    /// CHECK: This account is safe because we trust the vault's token account authority.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,

    /// CHECK: Using UncheckedAccount since we're only using it for transfer [(1)](https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens)
    #[account(mut)]
    pub user_token_account: UncheckedAccount<'info>,

    /// CHECK: Using UncheckedAccount for mint since we only need decimals [(1)](https://www.anchor-lang.com/docs/tokens/basics/transfer-tokens)
    pub mint: UncheckedAccount<'info>,
    
    pub token_program: Interface<'info, TokenInterface>,

    #[account(mut)]
    pub presale: SystemAccount<'info>,
}

#[error_code]
pub enum PresaleError {
    #[msg("You must send SOL to buy SLMT.")]
    InsufficientPayment,

    #[msg("Overflow occurred during SLMT calculation.")]
    Overflow,
}