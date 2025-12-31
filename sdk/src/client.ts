import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";
import { DiveOptions, DiveResult } from "./types";
import { BASE_WIRE_LENGTH } from "./constants";

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

    try {
      return await this.program.methods
        .initialize(bump)
        .accounts({
          protocolState: protocolPda,
          authority: authority,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();
    } catch (error) {
      throw new Error(`Failed to initialize: ${error}`);
    }
  }

  async createDive(options: DiveOptions): Promise<DiveResult> {
    if (options.wireLength < BASE_WIRE_LENGTH) {
      throw new Error(`Wire length must be at least ${BASE_WIRE_LENGTH}`);
    }

    const diver = this.provider.wallet.publicKey;
    const [protocolPda] = this.getProtocolPda();
    const [divePda] = this.getDivePda(diver);

    try {
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
    } catch (error) {
      throw new Error(`Failed to create dive: ${error}`);
    }
  }

  async spoolUp(diveAddress: web3.PublicKey, priority = false): Promise<string> {
    const diver = this.provider.wallet.publicKey;
    const [protocolPda] = this.getProtocolPda();

    try {
      return await this.program.methods
        .spoolUp(priority)
        .accounts({
          protocolState: protocolPda,
          diveState: diveAddress,
          diver: diver,
        })
        .rpc();
    } catch (error) {
      throw new Error(`Failed to spool up: ${error}`);
    }
  }

  async abortDive(diveAddress: web3.PublicKey): Promise<string> {
    const diver = this.provider.wallet.publicKey;

    try {
      return await this.program.methods
        .abortDive()
        .accounts({
          diveState: diveAddress,
          diver: diver,
        })
        .rpc();
    } catch (error) {
      throw new Error(`Failed to abort dive: ${error}`);
    }
  }

  async getProtocolState() {
    const [protocolPda] = this.getProtocolPda();
    return await this.program.account.protocolState.fetch(protocolPda);
  }

  async getDiveState(diveAddress: web3.PublicKey) {
    return await this.program.account.diveState.fetch(diveAddress);
  }
}
