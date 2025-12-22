import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { DiveOptions, DiveResult } from "./types";

export class DivwClient {
  private readonly PROTOCOL_SEED = Buffer.from("protocol");
  private readonly DIVE_SEED = Buffer.from("dive");

  constructor(
    public readonly program: Program,
    public readonly provider: AnchorProvider = program.provider as AnchorProvider
  ) {}

  getProtocolPda(): [web3.PublicKey, number] {
    return web3.PublicKey.findProgramAddressSync(
      [this.PROTOCOL_SEED],
      this.program.programId
    );
  }

  getDivePda(diver: web3.PublicKey): [web3.PublicKey, number] {
    return web3.PublicKey.findProgramAddressSync(
      [this.DIVE_SEED, diver.toBuffer()],
      this.program.programId
    );
  }

  async initialize(authority: web3.PublicKey): Promise<string> {
    const [protocolPda, bump] = this.getProtocolPda();

    return await this.program.methods
      .initialize(bump)
      .accounts({
        protocolState: protocolPda,
        authority: authority,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
  }

  async createDive(options: DiveOptions): Promise<DiveResult> {
    const diver = this.provider.wallet.publicKey;
    const [protocolPda] = this.getProtocolPda();
    const [divePda] = this.getDivePda(diver);

    const signature = await this.program.methods
      .createDive(options.depth, new BN(options.wireLength))
      .accounts({
        protocolState: protocolPda,
        diveState: divePda,
        diver: diver,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    return {
      signature,
      diveAddress: divePda,
      depth: options.depth,
      wireLength: options.wireLength,
    };
  }

  async spoolUp(diveAddress: web3.PublicKey, priority = false): Promise<string> {
    const diver = this.provider.wallet.publicKey;
    const [protocolPda] = this.getProtocolPda();

    return await this.program.methods
      .spoolUp(priority)
      .accounts({
        protocolState: protocolPda,
        diveState: diveAddress,
        diver: diver,
      })
      .rpc();
  }

  async abortDive(diveAddress: web3.PublicKey): Promise<string> {
    const diver = this.provider.wallet.publicKey;

    return await this.program.methods
      .abortDive()
      .accounts({
        diveState: diveAddress,
        diver: diver,
      })
      .rpc();
  }
}
