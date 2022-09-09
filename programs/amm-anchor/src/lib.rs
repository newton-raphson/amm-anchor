use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, MintTo,
    mint_to}};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAMM>) -> Result<()> {
        let bump = ctx.bumps.get("amm_authority").unwrap().to_le_bytes();
        let amm_data_key = ctx.accounts.amm_data.key().clone();              
        let inner = vec![
            amm_data_key.as_ref(),
            bump.as_ref(),
        ];
        let sig = vec![inner.as_slice()];
        let mint_instruction = MintTo {
            mint: ctx.accounts.pool_mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.amm_authority.to_account_info()
        };
        let cpi_mint = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info().clone(), mint_instruction,sig.as_slice());
        mint_to(cpi_mint, 1)?;
        let amm = &mut ctx.accounts.amm_data;
        amm.is_initialized = true;
        amm.token_a_account = *ctx.accounts.token_a.to_account_info().key;
        amm.token_b_account = *ctx.accounts.token_b.to_account_info().key;
        amm.pool_mint = *ctx.accounts.pool_mint.to_account_info().key;
        amm.token_a_mint = ctx.accounts.token_a.mint;
        amm.token_b_mint = ctx.accounts.token_b.mint;
        amm.pool_fee_account = *ctx.accounts.fee_account.to_account_info().key;
        // amm.fees = fees_input;
        // amm.curve = curve_input;
        Ok(())
    }


#[derive(Accounts)]
pub struct InitializeAMM<'info> {
     /// CHECK: Verified using seeds
    #[account(
        seeds = [
            amm_data.key().as_ref(),
        ],bump,
    )]
    pub amm_authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer=source_account,
        space=AmmData::MAX_SIZE,
    )]
    pub amm_data: Box<Account<'info, AmmData>>,
    #[account(mut)]
    pub source_account:Signer<'info>,
    #[account(
        init,
        payer = source_account,
        owner = token_program.key(),
        mint::decimals = 9,
        mint::authority = amm_authority,
    )]
    pub pool_mint: Account<'info, Mint>,
    // Token A token Account
    #[account(
        init_if_needed,
        payer = source_account,
        associated_token::mint = token_a_mint,
        associated_token::authority = amm_authority,
    )]
    pub token_a: Box<Account<'info, TokenAccount>>,
    /// CHECK: No checks required for the mint
    pub token_a_mint: Box<Account<'info, Mint>>,
    // Token B token Account
    #[account(
        init_if_needed,
        payer = source_account,
        associated_token::mint = token_b_mint,
        associated_token::authority = amm_authority,
    )]
    pub token_b: Box<Account<'info, TokenAccount>>,
    /// CHECK: No checks required for the mint
    pub token_b_mint: Box<Account<'info, Mint>>,
    // Fee Account for li
    #[account(
        init,
        payer = source_account,
        associated_token::mint = pool_mint,
        associated_token::authority = amm_authority,
    )]
    pub fee_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = source_account,
        associated_token::mint = pool_mint,
        associated_token::authority = source_account,
    )]
    pub destination: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program:Program<'info,Token>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program:Program<'info,AssociatedToken>,
}
#[account]
pub struct AmmData {
    /// is initialized
    pub is_initialized: bool,
    /// Token A Liquidity with Authority as AMM Authority
    pub token_a_account: Pubkey,
    /// Token B Liquidity with Authority as AMM Authority
    pub token_b_account: Pubkey,
    /// Liquidity Pool Mint: Mint authority is AMM Authority
    pub pool_mint: Pubkey,
    /// Token A mint
    pub token_a_mint: Pubkey,
    /// Token B mint
    pub token_b_mint: Pubkey,
    /// Fee Account for the pool
    pub pool_fee_account: Pubkey,
}
impl AmmData{

    pub const MAX_SIZE: usize = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 32;
}
#[account]
pub struct FeesInput {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}
#[account]
pub struct CurveInput {
    pub curve_type: u8,
    pub curve_parameters: u64,
}
}
pub fn build_fees(fees_input: &FeesInput) -> Result<FeesInput> {
    let fees = FeesInput {
        trade_fee_numerator: fees_input.trade_fee_numerator,
        trade_fee_denominator: fees_input.trade_fee_denominator,
        owner_trade_fee_numerator: fees_input.owner_trade_fee_numerator,
        owner_trade_fee_denominator: fees_input.owner_trade_fee_denominator,
        owner_withdraw_fee_numerator: fees_input.owner_withdraw_fee_numerator,
        owner_withdraw_fee_denominator: fees_input.owner_withdraw_fee_denominator,
        host_fee_numerator: fees_input.host_fee_numerator,
        host_fee_denominator: fees_input.host_fee_denominator,
    };
    Ok(fees)
}
