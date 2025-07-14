import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleMap} from "../target/types/simple_map";

describe("Simple Map", () => {
  const  provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SimpleMap as Program<SimpleMap>;

  it("Initialize", async () => {
    const user = provider.wallet;

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.initialize(new anchor.BN(23)).accounts({
      userAccount: userPda,
      authority: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    }).rpc();

    const account = await program.account.userAccount.fetch(userPda);
    console.log("📦 PDA:", userPda.toBase58());
    console.log("✅ Stroed Data:", account.data.toString());

  });

  it("Assigning new value", async () => {
    const user = provider.wallet;

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.set(new anchor.BN(99)).accounts({
      userAccount: userPda,
      authority: user.publicKey
    }).rpc();

    const updated = await program.account.userAccount.fetch(userPda);
    console.log("🔁 Updated Value: ", updated.data.toString());

  });
}); 