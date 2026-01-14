# DIVW SDK Reference

## Installation

```bash
npm install @divw/sdk
```

## DivwClient

Main client for interacting with the protocol.

### Methods

#### createDive(options)

Create a new dive operation.

```typescript
const dive = await client.createDive({
  depth: 5,
  wireLength: 200000,
});
```

#### spoolUp(diveAddress, priority?)

Execute pending transaction.

```typescript
await client.spoolUp(dive.diveAddress, true);
```

#### abortDive(diveAddress)

Cancel a pending dive.

```typescript
await client.abortDive(dive.diveAddress);
```

## Utilities

### calculateRecommendedWire(depth)

Returns recommended wire length for given depth.

### formatDepth(depth)

Returns human-readable depth zone name.
