import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import {
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
  MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";
import { publicKey } from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import { DaddyInu, NFTData } from "../target/types/daddy_inu";

describe("daddy-inu", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const umi = createUmi("https://api.devnet.solana.com")
	.use(walletAdapterIdentity(signer))
	.use(mplTokenMetadata());

  console.log(umi);

  anchor.setProvider(provider);
  const program = anchor.workspace.DaddyInu as Program<DaddyInu>;
  const signer = provider.wallet;

  const mint = anchor.web3.Keypair.generate();

  // Derive the associated token address account for the mint
  const associatedTokenAccount = await getAssociatedTokenAddress(
    mint.publicKey,
    signer.publicKey
  );

  const masterEditionAccount = findMasterEditionPda(umi, {
    mint: publicKey(mint.publicKey),
  }).at(0);

  const metadataAccount = findMasterEditionPda(umi, {
    mint: publicKey(mint.publicKey),
  }).at(0);

  const metadata = {
    name: "Big Daddy Inu",
    symbol: "BDI",
    uri: "https://localhost:8000/art.png",
    mint_supply: 1,
    max_supply: 5000000000,
    creators: [],
  } as NFTData;

  it("Is initialized!", async () => {
    console.log("Fucked")
    try {
      const tx = await program.methods
        .initNFT(metadata)
        .accounts({
          signer: provider.publicKey,
          mint: mint.publicKey,
          associatedTokenAccount,
          metadataAccount,
          masterEditionAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([mint])
        .rpc();

      console.log(`mint nft tx: ${tx}`);
      console.log(`minted nft: ${mint.publicKey}`);
    } catch (e) {
      console.eror("Error in test", error);
    }
  });
});

