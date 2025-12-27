import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("divw", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("initializes the protocol", async () => {
    // Test will be implemented after IDL generation
    expect(true).to.be.true;
  });
});
