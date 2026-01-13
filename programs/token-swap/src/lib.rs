use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ self, Mint, TokenAccount, TokenInterface, TransferChecked };

pub mod state;
use state::*;

pub mod instructions;
use instructions::*;

pub mod constants;

declare_id!("3CkXZ1WBCxWwLD24NkaEJfkMd7NN27Qoyp8dHvTYzPAR");

#[program]
pub mod token_swap {
    use super::*;

     pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool::createPool(ctx)
    }
    pub fn swap_give_a_get_b(
        ctx: Context<Swap>,
        amount_a: u64,
    ) -> Result<()> {
        instructions::swap::swap_give_a_get_b(ctx, amount_a)
    }
    pub fn swap_give_b_get_a(
        ctx: Context<Swap>,
        amount_a: u64,
    ) -> Result<()> {
        instructions::swap::swap_give_b_get_a(ctx, amount_b)
    }   
}
