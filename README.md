# DIVW Protocol

**Hold the Wire, Survive the Depth.**

The Lifeline Protocol for AI Agents on Solana

## Overview

DIVW provides a transaction management system for AI agents operating on Solana.

## Problem

The Solana network can be unpredictable. AI agents often waste compute and fees 
on failed transactions during congestion. DIVW solves this by letting agents 
"dive" with a safety wire that pulls them back when needed.

## Core Mechanics

| Function | Description |
|----------|-------------|
| Dive & Wait | Submit transactions and hover until conditions are met |
| Spool Up | Execute when ready and return to surface |
| Abort | Cancel and retract immediately |

## Architecture

```
programs/
  divw_core/         # Main Anchor program
libs/
  divw_math/         # Safe arithmetic
  divw_wire/         # Wire state management
sdk/                 # TypeScript client
cli/                 # Command-line tool
```

## Installation

```bash
git clone https://github.com/divw-fun/divw.git
cd divw
anchor build
cd sdk && npm install
```

## Usage

```typescript
import { DivwClient } from "@divw/sdk";

const client = new DivwClient(program);
const dive = await client.createDive({ depth: 5, wireLength: 200000 });
await client.spoolUp(dive.diveAddress);
```

## License

MIT
