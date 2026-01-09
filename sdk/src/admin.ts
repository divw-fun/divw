import { Program, web3 } from "@coral-xyz/anchor";

export class AdminClient {
  constructor(private program: Program) {}

  async togglePause(authority: web3.PublicKey): Promise<string> {
    const [protocolPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      this.program.programId
    );

    return await this.program.methods
      .togglePause()
      .accounts({
        protocolState: protocolPda,
        authority,
      })
      .rpc();
  }

  async transferAuthority(
    authority: web3.PublicKey,
    newAuthority: web3.PublicKey
  ): Promise<string> {
    const [protocolPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      this.program.programId
    );

    return await this.program.methods
      .transferAuthority()
      .accounts({
        protocolState: protocolPda,
        authority,
        newAuthority,
      })
      .rpc();
  }
}
