/* use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instructions::{CreateMetadataAccountsV3, CreateMetadataAccountsV3InstructionArgs};
use mpl_token_metadata::types::{DataV2, Creator}; */

// declare_id!("AkJqBfjD7opvZmenWKGcATwD7kZXcEwp2k2yCDJjRVQ6");

/*
#[program]
pub mod nebulaforge_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
} */
/*
#[program]
pub mod nebulaforge_program {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, name: String, symbol: String, uri: String) -> Result<()> {
        let data = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 500, // 5% royalties
            creators: Some(vec![Creator {
                address: ctx.accounts.user.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        let args = CreateMetadataAccountsV3InstructionArgs {
            data,
            is_mutable: true,
            collection_details: None,
        };

        CreateMetadataAccountsV3::invoke_with_program_id(
            &ctx.accounts.token_metadata_program.key(),
            &[
                &ctx.accounts.metadata.to_account_info(),
                &ctx.accounts.nft_mint.to_account_info(),
                &ctx.accounts.user.to_account_info(), // mint authority
                &ctx.accounts.user.to_account_info(), // payer
                &ctx.accounts.user.to_account_info(), // update authority
                &ctx.accounts.system_program.to_account_info(),
            ],
            &args,
        )?;

        Ok(())
    }

    // Placeholder for list, buy, bid, fuse - implement similarly
    pub fn list_nft(_ctx: Context<ListNft>, _price: u64) -> Result<()> {
        // Transfer to escrow, create listing account
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, mint::decimals = 0, mint::authority = user)]
    pub nft_mint: Account<'info, Mint>,
    /// CHECK: Metadata PDA derived as [ 'metadata', program_id, mint_id ]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Metaplex program
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

// Add structs for other instructions...

#[account]
pub struct NftListing {
    pub owner: Pubkey,
    pub price: u64,
    pub bids: Vec<(Pubkey, u64)>,
    pub metadata_uri: String,
}

#[account]
pub struct Fusion {
    pub parent1: Pubkey,
    pub parent2: Pubkey,
    pub child_mint: Pubkey,
} */

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use mpl_token_metadata::instruction::{create_metadata_accounts_v3};
use mpl_token_metadata::state::{DataV2};

declare_id!("AkJqBfjD7opvZmenWKGcATwD7kZXcEwp2k2yCDJjRVQ6");

#[program]
pub mod nebulaforge_program {
    use super::*;
    use solana_program::pubkey::Pubkey as SolanaPubkey;

    pub fn mint_nft(ctx: Context<MintNft>, name: String, symbol: String, uri: String) -> Result<()> {
        let data = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 500,
            creators: Some(vec![Creator {
                address: ctx.accounts.user.key().to_pubkey(), // Fix type
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        let metadata_pda = Pubkey::find_program_address(
            &[
                b"metadata",
                mpl_token_metadata::ID.as_ref(),
                ctx.accounts.nft_mint.key().as_ref(),
            ],
            &mpl_token_metadata::ID,
        ).0;

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.user.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    update_authority: ctx.accounts.user.to_account_info(),
                },
            ),
            data,
            true,
            true,
            None,
        )?;

        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.owner = ctx.accounts.user.key();
        listing.price = price;
        listing.bids = vec![];
        listing.metadata_uri = "".to_string(); // Set from IPFS later
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, mint::decimals = 0, mint::authority = user)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, MplTokenMetadata>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = NftListing::LEN)]
    pub listing: Account<'info, NftListing>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NftListing {
    pub owner: Pubkey,
    pub price: u64,
    pub bids: Vec<(Pubkey, u64)>,
    pub metadata_uri: String,
}

impl NftListing {
    const LEN: usize = 8 + 32 + 8 + 4 + (32 + 8) * 10 + 4 + 200; // Discriminator + owner + price + bids vec (up to 10) + uri string
}

#[account]
pub struct Fusion {
    pub parent1: Pubkey,
    pub parent2: Pubkey,
    pub child_mint: Pubkey,
}