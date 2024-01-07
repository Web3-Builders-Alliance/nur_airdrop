use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use create::state::Escrow;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    mint_a: Account<'info, Mint>,
    mint_b: Account<'info, Mint>,
    #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    )]
    maker_ata_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        Space = INIT_SPACE,
        seeds = [b"signer",
        makerkey.as_ref(),
        seed.to_le_bytes()],
        bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],  
        bump,
        token::mint = mint_a,
        token::authority = escrow,
    )]
   
    vault: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]

pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    mint_a: Account<'info, Mint>,
    #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    )]
    maker_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        Space = INIT_SPACE,
        seeds = [b"signer",
        makerkey.as_ref(),
        escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        seeds = [b"vault", escrow.key().as_ref()],  
        bump,
        token::mint = mint_a,
        token::authority = escrow,
        
    )]
   
    vault: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Escrow {
    seed: u64,
    mint_a: Pubkey,
    mint_b: Pubkey,
    receive: u64,
    bump: u8,
    vault_bump: u8,

}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 8+1 +1;
}

impl <'info> Make <'info> {
    pub fn save (&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {}
    self.escrow.set_inner (Escrow{
        seed,
        mint_a: self.mint_a.key(),
        mint_b: self.mint_b.key(),
        receive,
        bump: ctx.bumps.escrow,
        vault_bump: ctx.bumps.vault,
})
        Ok(())
    }

    pub fn deposit (&mut self, seed: u64, receive: u64, bumps: &MakeBumps) 
    let transfer_accounts = Transfer {
        from: self.maker_ata_a.to_account_info(),
        to: self.vault.to_account_info(),
        authority: self.maker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        self.token_program.to_account_info(),
        transfer_accounts,
    );
    
    transfer(cpi_ctx, deposit)  
}
}