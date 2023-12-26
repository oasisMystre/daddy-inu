import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";

import {
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
  MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";
import { publicKey } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";

import { DaddyInu, NFTData } from "../target/types/daddy_inu";

describe("daddy-inu", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const mint = anchor.web3.Keypair.generate();
  const program = anchor.workspace.DaddyInu as Program<DaddyInu>;

  const signer = provider.wallet;
  anchor.setProvider(provider);

  const umi = createUmi("https://api.devnet.solana.com")
    .use(walletAdapterIdentity(signer))
    .use(mplTokenMetadata());

  // Derive the associated token address account for the mint
  const associatedTokenAccount = await getAssociatedTokenAddress(
    mint.publicKey,
    signer.publicKey
  );

  const masterEditionAccount = findMasterEditionPda(umi, {
    mint: publicKey(mint.publicKey),
  }).at(0);

  const metadataAccount = findMetadataPda(umi, {
    mint: publicKey(mint.publicKey),
  }).at(0);

  const metadata = {
    creators: [],
    symbol: "BDI",
    name: "Big Daddy Inu",
    mint_supply: 1,
    seller_fee_basis_points: 0,
    max_supply: 5000000000,
    uri: "https://localhost:8000/art.png",
  };

  it("Is initialized!", async () => {
    console.log(MPL_TOKEN_METADATA_PROGRAM_ID);
    try {
      const tx = await program.methods
        .mintNft(metadata)
        .accounts({
          signer: provider.publicKey,
          mint: mint.publicKey,
          associatedTokenAccount,
          metadataAccount,
          masterEditionAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([mint])
        .rpc();

      console.log(`mint nft tx: ${tx}`);
      console.log(`minted nft: ${mint.publicKey}`);
    } catch (e) {
      console.error("Error in test", e);
    }
  });
});

