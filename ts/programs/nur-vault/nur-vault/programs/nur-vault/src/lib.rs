use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

declare_id!("HqpHC54qib1S7yHVscMxPH46zBf5VoxXiMGzjr9yAVZf");

#[program]
pub mod nur_vault {

    use super::*;

    pub fn initialize(ctx: Context<Deposit>) -> Result<()> {
        ctx.accounts.deposit(LAMPORTS_PER_SOL)?;

        msg!("Nur Vault initialized");

        Ok(())
    }

    // pub fn initialize(ctx: Context<Withdraw>) -> Result<()> {
    //     ctx.accounts.withdraw(LAMPORTS_PER_SOL)?;

    //     msg!("Nur Vault initialized");

    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Deposit <'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"nur-vault"],
        bump,
    )]
vault: SystemAccount<'info>,
system_program: Program<'info, System>
}

// let seeds = &[
//             b"nur-vault",
//             &[ctx.accounts.state.vault_bump],
//         ];

//         let pda_signer = &[&seeds[..]];

// pub struct Withdraw <'info> {
//     #[account(mut)]
//     signer: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [b"nur-vault"],
//         bump,
//     )]
// vault: SystemAccount<'info>,
// system_program: Program<'info, System>
// }

impl<'info> Deposit <'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

// impl<'info> Withdraw <'info> {
//     pub fn withdraw (&mut self, amount: u64) -> Result<()> {
//         let cpi_program = self.system_program.to_account_info();
//         let cpi_accounts = Transfer {
//             from: self.signer.to_account_info(),
//             to: self.vault.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

//         transfer(cpi_ctx, amount)?;

//         Ok(())
//     }
// }