use anchor_lang::prelude::*;

pub use anchor_spl::token::{Token, Mint};

pub use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize {
   #[account(mut)]
   pub admin: Signer<'info>,
   #[account(
    init, 
    payer = admin, 
    seeds = [b"marketplace", name.as_str().as_bytes()],
    bump,
    space = MARKETPLACE::INIT_SPACE + name.len(),
)]
pub marketplace: Account<'info, Marketplace>,
#[account(
    init, 
    payer = admin,
    seeds = [b"fee_vault", marketplace.key().as_ref()],
    bump,
)]
pub fee_vault: SystemAccount<'info>,
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn init(
    &mut self,
    name: String,
    fee: u16,
    ) -> Result <()> {
    self.marketplace.set_inner(
        MARKETPLACE {
            admin: self.admin.key(),
            fee,
            name,
        }
    );
    Ok(())
    }
 }
