import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { DiveOptions, DiveResult } from "../types";
import { BASE_WIRE_LENGTH } from "../constants";

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

  async createDive(options: DiveOptions): Promise<DiveResult> {
    if (options.wireLength < BASE_WIRE_LENGTH) {
      throw new Error("Wire length must be at least " + BASE_WIRE_LENGTH);
    }

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
}
