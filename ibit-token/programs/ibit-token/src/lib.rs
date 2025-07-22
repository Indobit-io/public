// File: programs/ibit-token/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, InitializeMint};

declare_id!("11111111111111111111111111111111111111111111");

#[program]
pub mod ibit_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let authority = &ctx.accounts.authority;

        // Total supply: 1_000_000_000 * 10^6 (6 decimals)
        let total = 1_000_000_000_000_000u64;

        // Allocation map
        let allocations = [
            (&ctx.accounts.founders, total * 30 / 100),
            (&ctx.accounts.treasury, total * 30 / 100),
            (&ctx.accounts.liquidity, total * 20 / 100),
            (&ctx.accounts.marketing, total * 10 / 100),
            (&ctx.accounts.rnd, total * 10 / 100),
        ];

        for (acct, amount) in allocations {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: mint.to_account_info(),
                    to: acct.to_account_info(),
                    authority: authority.to_account_info(),
                },
            );
            token::mint_to(cpi_ctx, amount)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub founders: Account<'info, TokenAccount>,

    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub treasury: Account<'info, TokenAccount>,

    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub liquidity: Account<'info, TokenAccount>,

    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub marketing: Account<'info, TokenAccount>,

    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub rnd: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
