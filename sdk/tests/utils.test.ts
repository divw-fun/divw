import {
  calculateRecommendedWire,
  formatDepth,
  isValidDepth,
  isValidWireLength,
} from "../src/utils";

describe("calculateRecommendedWire", () => {
  it("returns base wire for depth 1", () => {
    expect(calculateRecommendedWire(1)).toBe(100_000);
  });

  it("scales exponentially with depth", () => {
    expect(calculateRecommendedWire(2)).toBe(150_000);
    expect(calculateRecommendedWire(3)).toBe(225_000);
  });
});

describe("formatDepth", () => {
  it("returns correct zone names", () => {
    expect(formatDepth(1)).toBe("Surface");
    expect(formatDepth(5)).toBe("Deep");
    expect(formatDepth(10)).toBe("Mariana");
  });
});

describe("isValidDepth", () => {
  it("returns true for valid depths", () => {
    expect(isValidDepth(1)).toBe(true);
    expect(isValidDepth(10)).toBe(true);
  });

  it("returns false for invalid depths", () => {
    expect(isValidDepth(0)).toBe(false);
    expect(isValidDepth(11)).toBe(false);
    expect(isValidDepth(1.5)).toBe(false);
  });
});

describe("isValidWireLength", () => {
  it("validates minimum wire length", () => {
    expect(isValidWireLength(100_000)).toBe(true);
    expect(isValidWireLength(99_999)).toBe(false);
  });
});
