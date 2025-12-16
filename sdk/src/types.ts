import { web3 } from "@coral-xyz/anchor";

export enum DiveStatus {
  Hovering = 0,
  Surfaced = 1,
  Aborted = 2,
}

export interface DiveOptions {
  depth: number;
  wireLength: number;
  priority?: boolean;
}

export interface DiveResult {
  signature: string;
  diveAddress: web3.PublicKey;
  depth: number;
  wireLength: number;
}

export interface ProtocolStats {
  totalDives: number;
  successfulSpools: number;
  isActive: boolean;
}
