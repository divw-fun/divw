import { BASE_WIRE_LENGTH, WIRE_MULTIPLIER, MAX_DEPTH, MIN_DEPTH } from "./constants";

export function calculateRecommendedWire(depth: number): number {
  return Math.floor(BASE_WIRE_LENGTH * Math.pow(WIRE_MULTIPLIER, depth - 1));
}

export function formatDepth(depth: number): string {
  const zones = [
    "Surface",
    "Shallow",
    "Mid-water",
    "Thermocline",
    "Deep",
    "Twilight",
    "Midnight",
    "Abyssal",
    "Hadal",
    "Mariana",
  ];
  return zones[Math.min(depth - 1, zones.length - 1)] || "Unknown";
}

export function isValidDepth(depth: number): boolean {
  return Number.isInteger(depth) && depth >= MIN_DEPTH && depth <= MAX_DEPTH;
}

export function isValidWireLength(wireLength: number): boolean {
  return Number.isInteger(wireLength) && wireLength >= BASE_WIRE_LENGTH;
}
