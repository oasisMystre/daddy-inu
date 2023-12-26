use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        Metadata,
        CreateMetadataAccountsV3,
        CreateMasterEditionV3, 
        mpl_token_metadata::types::{DataV2, Creator},
    },
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use anchor_spl::{
    token::{mint_to, MintTo}, 
    metadata::{create_metadata_accounts_v3, create_master_edition_v3}
};
use mpl_token_metadata::pda::{find_master_edition_account, find_metadata_account};

declare_id!("4JTpmcGZjrTeS6QzQBzjsG3B4t1kEbR2JD9uspnhARdP");

#[program]
pub mod daddy_inu {
    use super::*;

    pub fn mint_nft(ctx: Context<Initialize>, data: NFTData) -> Result<()> {
        msg!("Initializing Mint Ticket");

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(), // Fix the typo here
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, data.mint_supply)?;

        msg!("Token Minted !!!");

        let cpi_context = CpiContext::new(ctx.accounts.token_metadata_program.to_account_info(), 
            CreateMetadataAccountsV3{
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            }
         );

        let data_v2 = DataV2{
            name: data.name,
            symbol: data.symbol,
            uri: data.uri,
            creators: 
            data.creators.map(|creators| {
                creators.into_iter().map(|creator| Creator {
                    address: creator.address,
                    share: creator.share,
                    verified: creator.verified,
                }).collect()
            }),            
            collection: None,
            uses: None,
            seller_fee_basis_points: data.seller_fee_basis_points,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;
        
        msg!("Metadata Account Created !!!");

        let cpi_context = CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(), 
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                update_authority: ctx.accounts.signer.to_account_info(),
                mint_authority: ctx.accounts.signer.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        );

        create_master_edition_v3(cpi_context, data.max_supply)?;

        msg!("Master Edition Nft Minted !!!");

        Ok(())
    }
}

/// When using Creator Struct IDL does not emit struct type
/// So this is a temporary fix
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct SerializedCreator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NFTData {
    name: String,
    symbol: String,
    uri: String,
    mint_supply: u64,
    max_supply: Option<u64>,
    seller_fee_basis_points: u16, 
    creators: Option<Vec<SerializedCreator>>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        address=find_master_edition_account(&mint.key()).0)
    ]
     /// CHECK: This is not dangerous because we don't read or write from this account
    pub master_edition_account: AccountInfo<'info>,
    #[account(
        mut,   
        address=find_metadata_account(&mint.key()).0,
    )]
     /// CHECK: This is not dangerous because we don't read or write from this account
    pub metadata_account: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
