
pub use anchor_lang::prelude::*;
pub use solana_program::sysvar::instructions::ID as INSTRUCTIONS_ID*;

use anchor_spl::{
    token::{Mint, TokenAccount},
    metadata::{Metadata, MetadataAccount, MasterEditionAccount,
    mpl_token_metadata::{
        instructions::{TransferCpi, TransferCpiAccounts, TransferInstructionArgs},
}},
    associated_token::{AssociatedToken, AssociatedTokenAccount},
};

pub use anchor_spl::token::Token;

pub use crate::state::*;
pub use crate::errors::*;

#[derive(Accounts)]
pub struct Delist <'info>{
   #[account(mut)]
   pub lister: Signer<'info>,
   #[account(
    init_if_needed,
    payer = lister,
    associated_token::mint = nft.key(),
    associated_token::authority = lister,
)]
pub lister_ata: Account<'info, TokenAccount>,

#[account(
seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
bump,
)]

pub marketplace: Account<'info, Marketplace>,
#[account( 
    seeds = [b"listing", marketplace.key().as_ref()],
    bump,
    has_one = lister,
    has_one = nft,
)]
pub listing: Account<'info, Listing>,
#[account(
    mut,
    payer = lister,
    associated_token::mint = nft,
    associated_token::authority = listing,
)]
pub listing_vault: Account<'info, TokenAccount>,

#[account(mut)]
pub nft: Account<'info, Mint>,
#[account(
    mut,
     seeds = [
         b"metadata", 
         token_metadata_program.key().as_ref(),
         nft.key().as_ref(),
     ],
        seeds::program = token_metadata_program.key(),
        bump,
       
 )]

 pub metadata: Account<'info, MetadataAccount>,
 #[account(
    mut,
     seeds = [
         b"metadata", 
         token_metadata_program.key().as_ref(),
         nft.key().as_ref(),
         b"edition",
     ],
        seeds::program = token_metadata_program.key(),
        bump,
 )]

pub edition: Account<'info, MasterEditionAccount>,

#[account(address = )]
///CHECK: No need to check it out
pub associated_token_program: Program<'info, AssociatedToken>,
pub token_metadata_program: Account<'info, Metadata>,
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>,
}

imp <'info> Delist <'info> {
    pub fn Delist(
    &mut self,
    bumps: DelistBumps,
    ) -> Result <()> {

        require!(self.metadata.token_standard.clone().unwrap() == TokenStandard::NonFungible, MarketplaceError::InvalidTokenStandard);
        require!(self.metadata.data.collection.clone().unwrap() == Collection{verified: true, pubkey: self.collection.key()}, MarketplaceError::InvalidCollection);

    self.listing.set_inner(
        Listing {
            owner: self.lister.key(),
            mint: self.nft.key(),
            collection: self.metadata.data.collection.key(),
            price,
        }
    );

    let transfer_program: AccountInfo = self.token_program.to_account_info();
    let token = &self.listing_vault.to_account_info();
    let token_owner = &self.listing.to_account_info();
    let destination_token = &self.lister_ata.to_account_info();
    let destination_owner = &self.lister.to_account_info();
    let mint = &self.nft.to_account_info();
    let metadata = &self.metadata.to_account_info();
    let edition = Some(&self.edition.to_account_info());
    let token_record = None;        
    let destination_token_record = None;
    let authority = &self.listing.to_account_info();
    let payer = &self.lister.to_account_info();
    let system_program = &self.system_program.to_account_info();
    let sysvar_instructions = &self.sysvar_instruction.to_account_info();
    let spl_token_program = &self.token_program.to_account_info();
    let spl_ata_program = &self.associated_token_program.to_account_info();
    let authorization_rules_program = None;
    let authorization_rules = None;

    let transfer_cpi = TransferCpi::new(
    &transfer_program,
    accounts: TransferCpiAccounts {
        token,
        token_owner,
        destination_token,
        destination_owner,
        mint,
        metadata,
        edition,
        token_record,
        destination_token_record, 
        authority,
        payer,
        system_program,
        sysvar_instructions,
        spl_token_program,
        spl_ata_program,
        authorization_rules_program,
        authorization_rules,
    },

    args: TransferInstructionArgs {
        transfer_args: TransferArgs::V1{
            amount: 1,
            authorization_rules: None,
        },
    }
);

let seed =  &[
b"listing",
self.marketplace.key().as_ref(),
&[bumps.listing],
];

let marketplace_key = self.marketplace.key();
let signer_seeds = &[&seed[..]];
transfer_cpi.invoke_signed(signer_seeds)?;
    
    Ok(())
    }
 }