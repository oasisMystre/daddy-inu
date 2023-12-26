// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::pubkey::Pubkey;
// use anchor_spl::associated_token::AssociatedToken;
// use anchor_spl::metadata::Metadata;
// use anchor_spl::solana_program::sysvar::rent::Rent;
// use anchor_spl::system::System;
// use anchor_spl::token::Mint;
// use anchor_spl::token::Token;

// // Import your program module
// use crate::daddy_inu;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anchor_lang::test::*;
//     use anchor_spl::solana_program::sysvar::rent::create_account_rent;

//     #[tokio::test]
//     async fn test_initialize() {
//         // Set up testing environment
//         let mut test_program = program::start_test().await;
//         let test_keys = &mut test_program.keys;

//         // Generate test accounts
//         let signer = Keypair::new();
//         let mint = test_keys.new_account::<Mint>(&signer);
//         let associated_token_account = test_keys.new_account::<AssociatedToken>(&signer);
//         let master_edition_account = test_keys.new_account::<Account>(&signer);
//         let metadata_account = test_keys.new_account::<Metadata>(&signer);

//         // Create accounts needed for the CPI context
//         let token_program = test_keys.create_program_account::<Token>().await;
//         let metadata_program = test_keys.create_program_account::<Metadata>().await;
//         let associated_token_program = test_keys.create_program_account::<AssociatedToken>().await;
//         let system_program = test_keys.create_program_account::<System>().await;

//         // Create a test rent sysvar
//         let rent_sysvar = test_keys.create_program_account::<Rent>().await;
//         create_account_rent(&test_program.program_id, &rent_sysvar, 1, 0, 0).unwrap();

//         // Set up Initialize context
//         let ctx = test_program
//             .accounts
//             .initialize(
//                 test_program.program_id.clone(),
//                 signer.clone(),
//                 mint.clone(),
//                 associated_token_account.clone(),
//                 master_edition_account.clone(),
//                 metadata_account.clone(),
//                 rent_sysvar.clone(),
//                 token_program.clone(),
//                 system_program.clone(),
//                 metadata_program.clone(),
//                 associated_token_program.clone(),
//             )
//             .await
//             .unwrap();

//         // Set up NFTData for initialization
//         let data = NFTData {
//             name: "Test NFT".to_string(),
//             symbol: "TST".to_string(),
//             uri: "https://example.com/test".to_string(),
//             mint_supply: 10,
//             max_supply: Some(100),
//             seller_fee_basis_points: 500,
//             creators: None,
//         };

//         // Call the program's initialize function
//         daddy_inu::mint_nft(ctx, data).unwrap();

//         // Add assertions based on your program's logic
//         // For example, check if associated_token_account balance is correct after minting

//         // ...
//     }
// }
