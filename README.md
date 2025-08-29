# GAP ECS PoC

**Game Agent Protocol (GAP) – ECS Proof-of-Concept**

This repo explores using a small ECS-powered game world to prototype the **GAP protocol**:
- Publish game state via WebSocket at 30 Hz
- Accept simple `intent` messages (`move`, `say`, `use_potion`)
- Run local/AI agents via Python or Rust clients

---
## The SPEC

- **GAP** see [GAP.md](GAP.md)
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

