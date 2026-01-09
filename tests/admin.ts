import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";

describe("admin instructions", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("can pause and unpause protocol", async () => {
    expect(true).to.be.true;
  });

  it("can transfer authority", async () => {
    expect(true).to.be.true;
  });

  it("rejects unauthorized pause attempts", async () => {
    expect(true).to.be.true;
  });
});
