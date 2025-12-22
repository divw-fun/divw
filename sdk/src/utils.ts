/**
 * Calculate recommended wire length based on depth
 */
export function calculateRecommendedWire(depth: number): number {
  const BASE_WIRE = 100_000;
  return Math.floor(BASE_WIRE * Math.pow(1.5, depth - 1));
}

/**
 * Format depth level to human-readable zone name
 */
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

/**
 * Validate depth parameter
 */
export function isValidDepth(depth: number): boolean {
  return Number.isInteger(depth) && depth >= 1 && depth <= 10;
}

/**
 * Validate wire length
 */
export function isValidWireLength(wireLength: number): boolean {
  return Number.isInteger(wireLength) && wireLength >= 100_000;
}
