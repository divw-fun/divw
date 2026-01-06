<div align="center">

# DIVW Protocol

**Hold the Wire, Survive the Depth.**

<p>
  <img src="https://img.shields.io/badge/Solana-Mainnet-14F195?style=for-the-badge&logo=solana" alt="Solana" />
  <img src="https://img.shields.io/badge/Anchor-0.29.0-7C3AED?style=for-the-badge" alt="Anchor" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License" />
</p>

*The Lifeline Protocol for AI Agents on Solana*

</div>

---

## Overview

DIVW provides a transaction management system for AI agents on Solana. Agents can 
submit transactions in advance and execute them when network conditions are favorable.

## Problem

AI agents on Solana often fail during network congestion, wasting compute and fees.
DIVW provides a "wire" that keeps agents connected to safety while they operate in 
the depths of the network.

## Core Mechanics

| Function | Description |
|----------|-------------|
| **Dive & Wait** | Submit and hover until conditions are met |
| **Spool Up** | Execute and return to surface |
| **Abort** | Cancel and retract immediately |

## Token Utility ($DIVW)

| Utility | Description |
|---------|-------------|
| **Wire Extension** | Longer wires for complex transactions |
| **Priority Spooling** | First in queue during congestion |

## Installation

```bash
git clone https://github.com/divw-fun/divw.git
cd divw

# Build programs
anchor build

# Install SDK
cd sdk && npm install
```

## Usage

```typescript
import { DivwClient, calculateRecommendedWire } from "@divw/sdk";

const client = new DivwClient(program);

const dive = await client.createDive({
  depth: 5,
  wireLength: calculateRecommendedWire(5),
});

await client.spoolUp(dive.diveAddress, true);
```

## Project Structure

```
programs/divw_core/    Core Anchor program
libs/divw_math/        Safe arithmetic
libs/divw_wire/        Wire state management
sdk/                   TypeScript client
cli/                   Command-line tool
tests/                 Integration tests
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT

---

<div align="center">
  <strong>Dive Deep. Stay Connected.</strong>
  <br><br>
  <a href="https://divw.fun">Website</a> |
  <a href="https://x.com/divwdotfun">Twitter</a>
</div>
