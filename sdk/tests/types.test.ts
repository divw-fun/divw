import { DiveStatus } from "../src/types";
import { isValidDepth, isValidWireLength } from "../src/utils";

describe("Type validation", () => {
  describe("DiveStatus", () => {
    it("has correct enum values", () => {
      expect(DiveStatus.Hovering).toBe(0);
      expect(DiveStatus.Surfaced).toBe(1);
      expect(DiveStatus.Aborted).toBe(2);
    });
  });

  describe("Validation functions", () => {
    it("validates depth correctly", () => {
      expect(isValidDepth(0)).toBe(false);
      expect(isValidDepth(1)).toBe(true);
      expect(isValidDepth(10)).toBe(true);
      expect(isValidDepth(11)).toBe(false);
    });

    it("validates wire length correctly", () => {
      expect(isValidWireLength(99999)).toBe(false);
      expect(isValidWireLength(100000)).toBe(true);
      expect(isValidWireLength(1000000)).toBe(true);
    });
  });
});
