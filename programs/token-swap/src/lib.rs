use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };
declare_id!("3CkXZ1WBCxWwLD24NkaEJfkMd7NN27Qoyp8dHvTYzPAR");

#[program]
pub mod token_swap {
    use super::*;

    pub fn createPool(ctx: Context<CreatePool>) -> Result<()> {
        Ok(())
    }
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
        seeds = [b"pool"],
        bump,
    )]
    pub pool : Account<'info, Pool>, //its a pda to run forever and to be non-custodial
    
    pub system_program : Program<'info, System>,
    pub token_program : Interface<'info, TokenInterface>,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
}

