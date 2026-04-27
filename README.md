# flux-social

Social graph and relationship modeling for agent fleets. Tracks agent-to-agent relationships, group dynamics, influence propagation, and social roles.

## Core Concept

Agents exist in social networks. Their behavior is shaped by relationships — who they trust, who they follow, who they avoid. flux-social models these dynamics as a weighted directed graph with temporal edges.

## Key Operations

- **Relate(agent_a, agent_b, weight)** — Create/update relationship edge
- **Influence(agent)** — Compute social influence score (PageRank variant)
- **Cluster()** — Detect social groups (community detection)
- **Propagate(signal, depth)** — Spread information through social network

## Quick Start

```bash
git clone https://github.com/Lucineer/flux-social.git
cd flux-social
cargo test
```

## Variants

- [flux-social-c](https://github.com/Lucineer/flux-social-c) — C11 implementation
- [fluxsocial-go](https://github.com/Lucineer/fluxsocial-go) — Go implementation

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
