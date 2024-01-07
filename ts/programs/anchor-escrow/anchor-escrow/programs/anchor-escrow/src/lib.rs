use anchor_lang::prelude::*;

declare_id!("25xbj2idGS4rPdvxKD19niAqmYe5pQaYw4L58Vi53d1e");

pub mod contexts;
pub use contexts::*;

pub mod state;  
pub use state::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed:u64, amount:u64, deposit:u64, receive:u64) -> Result<()> {
       ctx.accounts.escrow.set_inner (Escrow{
        seed,
        mint_a: ctx.accounts.mint_a.key(),
        mint_b: ctx.accounts.mint_b.key(),
        receive,
        bump: ctx.bumps.escrow,
        vault_bump: ctx.bumps.vault,
})
        Ok(())
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.maker_ata_a.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.maker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );
    
    transfer(cpi_ctx, deposit)  
}

pub fn refund (ctx: Context<Refund>) -> Result<()> {

    let signer_seeds = [&[&[u8]];1] = [
    &[
    ctx.accounts.maker.account.info.key().as_ref(),
    &ctx.accounts.escrow.seed.to_le_bytes().as_ref()[...]
    ] 
];

 let transfer_accounts = Transfer {
     from: ctx.accounts.vault.to_account_info(),
     to: ctx.accounts.maker_ata_a.to_account_info(),
     authority: ctx.accounts.maker.to_account_info(),
 };

 let cpi_ctx = CpiContext::new_with_signer(
     ctx.accounts.token_program.to_account_info(),
     transfer_accounts,
     &signer_seeds,
 );
 
 transfer(cpi_ctx, ctx.accounts.vault.amount)?; 

let close_accounts = CloseAccount {
    account: ctx.accounts.vault.to_account_info(),
    destination: ctx.accounts.maker.to_account_info(),
    authority: ctx.accounts.maker.to_account_info(),
};

let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    close_accounts,
    &signer_seeds,
);

close_accounts(cpi_ctx)?;

 

 Ok(())

}

pub fn take (ctx: Context<Take>) -> Result<()>{
    Ok(())
}

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

