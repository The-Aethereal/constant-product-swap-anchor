use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenInterface, TokenAccount, InterfaceAccount};

use crate::state::pool::Pool;

pub fn provide_liquidity(ctx: Context<ProvideLiquidity>, amount_a: u64, amount_b: u64) -> Result<()>{
    let vault_a = &ctx.accounts.vault_a;
    let vault_b = &ctx.accounts.vault_b;

    let x = vault_a.amount as u128;
    let y = vault_b.amount as u128;

    let lp_mint = &ctx.accounts.lp_mint;
    let lp_supply = lp.mint.supply as u128;
    let dx = amount_a as u128;
    let dy = amount_b as u128;

    let lp_to_mint = if lp_supply == 0 {
        integer_sqrt(dx * dy)
    } else {
        let lp_from_a = dx * lp_supply / x;
        let lp_from_b = dy * lp_supply / y;
        lp_from_a.min(lp_from_b)
    };
    // Take in tokens

    // Mint LP tokens
}

#[derive(Account)]

pub struct ProvideLiquidity<'info>{
    #[account(mut)]
    pub user : Signer<'info>, // payer not owner
    #[account(
        seeds = POOL_SEED, 
        bump = pool.bump)]
    pub pool: Account<'info, Pool>, 
     #[account(
        mut,
        address = pool.mint_a,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.mint_b,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        address = pool.lp_mint,
    )]
    pub lp_mint : InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = pool.mint_a,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_a: InterfaceAccount<'info, TokenAccount>, 

     #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = pool.mint_b,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_b: InterfaceAccount<'info, TokenAccount>,

     #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = pool.lp_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_lp: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program : Program<'info, System>,
    pub token_program : Interface<'info, TokenInterface>, 
}