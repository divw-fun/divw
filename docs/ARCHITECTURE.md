# Architecture

## Overview

DIVW consists of three main components:

1. **On-chain Programs** (Solana/Anchor)
2. **TypeScript SDK**
3. **CLI Tool**

## Program Architecture

```
divw_core
├── instructions/
│   ├── initialize.rs
│   ├── create_dive.rs
│   ├── spool_up.rs
│   ├── abort_dive.rs
│   └── admin.rs
├── state/
│   ├── protocol.rs
│   └── dive.rs
├── events.rs
├── error.rs
└── constants.rs
```

## Data Flow

1. User calls SDK method
2. SDK constructs transaction
3. Transaction sent to Solana
4. Program validates and executes
5. State updated, event emitted
6. SDK returns result

## Account Structure

### ProtocolState PDA
- Seeds: ["protocol"]
- Stores global protocol configuration

### DiveState PDA
- Seeds: ["dive", diver_pubkey]
- Stores individual dive information
