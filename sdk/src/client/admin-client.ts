import { Program, AnchorProvider, web3 } from "@coral-xyz/anchor";

export class AdminClient {
  constructor(
    public readonly program: Program,
    public readonly provider: AnchorProvider = program.provider as AnchorProvider
  ) {}

  async togglePause(): Promise<string> {
    const [protocolPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      this.program.programId
    );

    return await this.program.methods
      .togglePause()
      .accounts({
        protocolState: protocolPda,
        authority: this.provider.wallet.publicKey,
      })
      .rpc();
  }

  async transferAuthority(newAuthority: web3.PublicKey): Promise<string> {
    const [protocolPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      this.program.programId
    );

    return await this.program.methods
      .transferAuthority()
      .accounts({
        protocolState: protocolPda,
        authority: this.provider.wallet.publicKey,
        newAuthority,
      })
      .rpc();
  }
}
