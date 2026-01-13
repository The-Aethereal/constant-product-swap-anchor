use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenInterface, TokenAccount, InterfaceAccount};

use crate::state::pool::Pool;

 pub fn swap_give_a_get_b(ctx: Context<Swap>, amount_a: u64) -> Result<()>{
        let vault_a = &ctx.accounts.vault_a;
        let vault_b = &ctx.accounts.vault_b;

        let x = vault_a.amount as u128;
        let y = vault_b.amount as u128;
        let dx = amount_a as u128;


        let dy = (y * dx) / (x + dx); //derived from constant product formula inspired from uniswap

       // Tranfer A from user to pool

        // Transfer B from pool to user
       

        Ok(())
    }
    pub fn swap_give_b_get_a(ctx: Context<Swap>, amount_b: u64) -> Result<()>{
        let vault_a = &ctx.accounts.vault_a;
        let vault_b = &ctx.accounts.vault_b;

        let x = vault_a.amount as u128;
        let y = vault_b.amount as u128;
        let dy = amount_b as u128;


        let dx = (x * dy) / (y + dy); 

       // Tranfer B from user to pool

        // Transfer A from pool to user
       

        Ok(())
    }


#[derive(Accounts)]
pub struct Swap<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"pool"], 
        bump = pool.bump)]
    pub pool: Account<'info, Pool>, 
     #[account(
        mut,
       address = pool.vault_a,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

     #[account(
        mut,
        address = pool.vault_b,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,
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
     pub system_program : Program<'info, System>,
    pub token_program : Interface<'info, TokenInterface>,
}

