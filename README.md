# flux-social 👥

**Social graph and trust-weighted relationship modeling for agent fleets.** Track agents, connections (Trust/Communication/Cooperation/Observation), groups, and influence propagation.

```rust
use flux_social::SocialGraph;

let mut g = SocialGraph::new();
g.add_agent(1, 0);  // id=1, group=0
g.add_agent(2, 0);
g.connect(1, 2, EdgeKind::Trust, 0.8);
g.set_trust(1, 2, 0.9);

let nbrs = g.neighbors(1);
println!("Agent 1 trust of 2: {}", g.trust(1, 2));
println!("Agent 2 influence: {}", g.influence(2));
```

## API

```rust
let mut g = SocialGraph::new();

// Agent management
g.add_agent(1, 0);             // id=1, group=0
g.remove_agent(1);
let group = g.group_of(1);     // Option<u8>

// Connections
g.connect(1, 2, EdgeKind::Trust, 0.8);
g.set_trust(1, 2, 0.95);       // upsert trust edge
g.disconnect(1, 2);
let t = g.trust(1, 2);         // trust from 1→2

// Social queries
let nbrs = g.neighbors(1);     // Vec<&Agent>
let inf = g.influence(2);      // sum of incoming edge weights
```

### Edge Types

| Kind | Purpose |
|------|---------|
| Trust | Reliability rating |
| Communication | Message frequency |
| Cooperation | Task collab strength |
| Observation | Monitoring relationship |

## Cargo.toml

```toml
[dependencies]
flux-social = { git = "https://github.com/Lucineer/flux-social" }
```

## Fleet Context

Part of the Lucineer/Cocapn fleet. Pairs with [flux-trust](https://github.com/Lucineer/flux-trust) (scoring engine) and [flux-telepathy](https://github.com/Lucineer/flux-telepathy) (message routing over the social graph). Also available in [C11](https://github.com/Lucineer/flux-social-c) and [Go](https://github.com/Lucineer/fluxsocial-go).
