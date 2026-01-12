# Admin Operations

## Pause/Unpause Protocol

The protocol can be paused by the authority in case of emergency.

```typescript
import { AdminClient } from "@divw/sdk";

const admin = new AdminClient(program);
await admin.togglePause(authority);
```

## Transfer Authority

Authority can be transferred to a new address.

```typescript
await admin.transferAuthority(currentAuthority, newAuthority);
```

## Security Considerations

- Only the current authority can execute admin operations
- Pausing stops all new dives but allows existing ones to complete
- Authority transfer is immediate and irreversible
