use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        Metadata,
        CreateMetadataAccountsV3,
        CreateMasterEditionV3, mpl_token_metadata::types::DataV2,
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

    pub fn initialize(ctx: Context<Initialize>, data: NFTData) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(), // Fix the typo here
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, 1)?;

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
            creators: None,
            collection: None,
            uses: None,
            seller_fee_basis_points: data.seller_fee_basis_points,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;
        

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

        create_master_edition_v3(cpi_context, Some(5000000))?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NFTData {
    name: String,
    symbol: String,
    uri: String,
    seller_fee_basis_points: u16,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
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
    pub master_edition_account: AccountInfo<'info>,
    #[account(
        mut,   
        address=find_metadata_account(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
