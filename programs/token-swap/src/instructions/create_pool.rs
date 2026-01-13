
use anchor_lang::prelude::*;
use anchor_spl::token::{InterfaceAccount,TokenInterface, TokenAccount, Mint};

use crate::state::pool::Pool;
use crate::constants::POOL_SEED;
pub fn createPool(ctx: Context<CreatePool>) -> Result<()> {
        *ctx.accounts.pool = Pool{
            mint_a : ctx.accounts.mint_a.key(),
            mint_b : ctx.accounts.mint_b.key(),
            vault_a : ctx.accounts.vault_a.key(),
            vault_b : ctx.accounts.vault_b.key(),
            bump : ctx.bumps.pool,
        } ;
        Ok(())
    }


#[derive(Accounts)]
pub struct CreatePool<'info> {
     #[account(mut)]
     pub payer : Signer<'info>, // payer not owner
    pub mint_a : InterfaceAccount<'info,Mint>, 
    pub mint_b : InterfaceAccount<'info,Mint>, 

     #[account(
        init,
        payer = payer,
        token::mint = mint_a,
        token::authority = pool
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = mint_b,
        token::authority = pool
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer= payer,
        space = 8+ Pool::INIT_SPACE,
        seeds = POOL_SEED,
        bump,
    )]
    pub pool : Account<'info, Pool>, //its a pda to run forever and to be non-custodial
    
    pub system_program : Program<'info, System>,
    pub token_program : Interface<'info, TokenInterface>,
}
