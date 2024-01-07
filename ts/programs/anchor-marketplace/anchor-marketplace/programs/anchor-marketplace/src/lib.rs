use anchor_lang::prelude::*;

pub mod context;
pub use context::*;
pub mod state;  
pub use state::*;
pub mod errors;
pub use errors::*;

declare_id!("EVP5pjb8nJAcYs52r7uFncy4oEcn1nGnwEY91CmYi9YY");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
