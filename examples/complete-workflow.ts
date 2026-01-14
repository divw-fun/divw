/**
 * Complete DIVW Workflow Example
 */

import { Connection, Keypair } from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { DivwClient, calculateRecommendedWire } from "@divw/sdk";

async function completeWorkflow() {
  console.log("=== DIVW Complete Workflow ===");
  console.log("");

  // 1. Setup
  console.log("1. Setting up connection...");
  const connection = new Connection("https://api.devnet.solana.com");

  // 2. Create dive
  console.log("2. Creating dive at depth 5...");
  const wireLength = calculateRecommendedWire(5);
  console.log("   Wire length: " + wireLength.toLocaleString() + " CU");

  // 3. Wait for conditions (simulated)
  console.log("3. Waiting for favorable conditions...");
  await new Promise((resolve) => setTimeout(resolve, 2000));

  // 4. Spool up
  console.log("4. Spooling up with priority...");

  // 5. Done
  console.log("");
  console.log("=== Workflow Complete ===");
}

completeWorkflow().catch(console.error);
