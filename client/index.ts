import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { DegenWall } from "./degen_wall";
import dotenv from "dotenv";
import { Keypair, PublicKey } from "@solana/web3.js";
dotenv.config();
const run = async () => {
  const SEED_PREFIX = Buffer.from("degen_wall");
  const mint = new PublicKey("A4SvyMLMGXrHR8ahP7qotUrKvGD8KgbdAcLNs3nbVftE");
  const MINT_SEED = mint.toBuffer();
  const discount = 98;
  const burn = false;
  const token = new PublicKey("A4SvyMLMGXrHR8ahP7qotUrKvGD8KgbdAcLNs3nbVftE"); // usually we should provide different token address for advertisement
  const website = "ZyKaSkiv6iBfIS8eT8j1xLeKAElLRHowytQak3HONvMPtWkky";
  const twitter = "12345678901234";
  const community = "ZyKaSkiv6iBfIS8eT8j1xLeKAElLRHowytQak3HONvMPtWkky";
  const image = "ZyKaSkiv6iBfIS8eT8j1xLeKAElLRHowytQak3HONvMPtWkky";
  const name = "a";
  const ticker = "ZyKaSki";
  const description = "ZyKaSkiv6iBfIS8eT8j1xLeKAE";
  const data = [...[5, 5, 255, 255, 255], ...new Array(495).fill(255)];
  const id = Array.from(Keypair.generate().publicKey.toBytes());
  const ID_SEED = Buffer.from(id);
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = anchor.getProvider().publicKey;
  if (!payer) return 0;
  const PAYER_SEED = payer.toBuffer();
  const program = anchor.workspace.DegenWall as Program<DegenWall>;
  const treasury = new PublicKey(
    "AWJQAWxPE3hJz2XVrJDmBDdQk4pC2SjeKpLFhjUncCKM"
  );
  const treasury_mint = new PublicKey(
    "CeqkbDdECYJZ86K4qJndBxNQD85tFj498XYY5UyxPuQp"
  );
  const vault_wsol = new PublicKey(
    "58DmxrkK8KTJkDhcG4oDVQ7Li7yfbpNikhtsiLD53KTD"
  );
  const vault_mint = new PublicKey(
    "6jBwx67VgAFPBTrGKnTmE2SVKzXgWJ2sAP25ks2YZUE1"
  );
  const payer_ata = new PublicKey(
    "5h8v7RfhTeEqTmhrxx462dudjkCThmZqywNaXNj2oxwH"
  );
  const [sol_treasury_account] = web3.PublicKey.findProgramAddressSync(
    [SEED_PREFIX],
    program.programId
  );
  const [pool_account] = web3.PublicKey.findProgramAddressSync(
    [SEED_PREFIX, MINT_SEED],
    program.programId
  );
  const [metadata_account] = web3.PublicKey.findProgramAddressSync(
    [SEED_PREFIX, PAYER_SEED, ID_SEED],
    program.programId
  );
  console.log("======================");
  console.log("Create Sol Treasury");
  try {
    await program.methods
      .createSolTreasuryAccount(discount)
      .accounts({
        authority: payer, //@ts-ignore
        solTreasuryAccount: sol_treasury_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Create Pool");
  try {
    await program.methods
      .createPoolAccount(discount, burn)
      .accounts({
        authority: payer,
        mint,
        vaultWsol: vault_wsol,
        vaultMint: vault_mint,
        treasury: treasury_mint, //@ts-ignore
        poolAccount: pool_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Update Pool");
  try {
    await program.methods
      .updatePoolAccount(Math.floor(discount + 1), burn)
      .accounts({
        authority: payer,
        mint, //@ts-ignore
        poolAccount: pool_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Create Metadata");
  try {
    await program.methods
      .createMetadataAccount({
        id,
        token,
        data,
        website,
        twitter,
        community,
        image,
        name,
        ticker,
        description,
      })
      .accounts({
        authority: payer, //@ts-ignore
        metadataAccount: metadata_account,
        solTreasuryAccount: sol_treasury_account,
        treasury: treasury,
        token: token,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Delete Metadata");
  try {
    await program.methods
      .deleteMetadataAccount(id)
      .accounts({
        authority: payer,
        payer: payer, //@ts-ignore
        metadataAccount: metadata_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Create Metadata Mint");
  try {
    await program.methods
      .createMetadataAccountMint({
        id,
        token,
        data,
        website,
        twitter,
        community,
        image,
        name,
        ticker,
        description,
      })
      .accounts({
        authority: payer, //@ts-ignore
        metadataAccount: metadata_account,
        mint,
        poolAccount: pool_account,
        treasuryMint: treasury_mint,
        payerTokenAccount: payer_ata,
        vaultWsol: vault_wsol,
        vaultMint: vault_mint,
        token: token,
      })
      .rpc();
    console.log("Success");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Delete Metadata Mint");
  try {
    await program.methods
      .deleteMetadataAccount(id)
      .accounts({
        authority: payer,
        payer: payer, //@ts-ignore
        metadataAccount: metadata_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");

  console.log("Delete Pool");
  try {
    await program.methods
      .deletePoolAccount()
      .accounts({
        authority: payer,
        mint, //@ts-ignore
        poolAccount: pool_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }
  console.log("======================");
  console.log("Delete Sol Treasury");
  try {
    await program.methods
      .deleteSolTreasuryAccount()
      .accounts({
        authority: payer, //@ts-ignore
        solTreasuryAccount: sol_treasury_account,
      })
      .rpc();
    console.log("Success!");
  } catch (error) {
    console.error(error);
  }

};

run();
