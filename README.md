# GAP ECS PoC

**Game Agent Protocol (GAP) – ECS Proof-of-Concept**

This repo holds the **GAP protocol** + a small ECS-powered reference world to
prototype it against:
- Publish game state via WebSocket at 30 Hz
- Accept simple `intent` messages (`move`, `say`, `use_potion`)
- Run local/AI agents via Python or Rust clients

GAP is a **protocol + runtime**; games attach via **adapters**. The first adapter
(DevilutionX) is where the architecture earned its scars — this repo is the
clean-room runtime where it stays honest.

> **New here? Start with [GETTING-STARTED.md](GETTING-STARTED.md)** — what GAP is,
> how to run the reference world, and the hard-won lessons from the first adapter
> that correct the original spec assumptions.

---
## The SPEC

- **GAP** see [GAP.md](GAP.md) — protocol spec (v0.4 draft; sections marked
  ⚠️ SUPERSEDED are v0.3 assumptions the first adapter overturned).
## License

- **Code:** Licensed under [Apache-2.0](LICENSE)
- **Specification:** Licensed under [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)

You are free to use, modify, and adapt this project for any purpose, even commercially, as long as attribution is given for the spec.

---

## Quick Start

Requirements:
- [Rust](https://www.rust-lang.org/) 1.75+
- [Python](https://www.python.org/) 3.10+
- [WebSocket client libraries](https://pypi.org/project/websockets/) for Python agent

Run the Bevy ECS world:

```bash
cargo run
# new shell
python scripts/run_agent.py
```

