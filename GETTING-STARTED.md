# Getting Started with GAP

> For future us. Read this before touching the spec or writing a new adapter — it
> captures what the first real implementation taught us, most of which contradicts
> the tidy assumptions in the original [GAP.md](GAP.md) draft.

## What GAP actually is

GAP is **not a Diablo mod**. It's a **protocol + runtime** for letting AI agents
play games cooperatively with humans. A game plugs in through an **adapter**:

```
                         GAP
                 (protocol + runtime)
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   DevilutionX        Bevy ECS           Godot / Unity / …
    adapter           reference world      (future adapters)
  (first adapter,    (this repo —
   "the wind tunnel") clean-room runtime)
```

**Diablo (DevilutionX) is the reference implementation, not the product** — a
deliberately chosen "wind tunnel": a small, understandable engine with
deterministic mechanics, a real networking model, decades of known behavior, and
fast iteration. We built the first adapter there to *discover* the architecture.
This repo is the clean-room runtime (a Bevy ECS world) where the protocol gets to
exist independent of any one game.

The pipeline is game-agnostic. The council doesn't know it's playing Diablo:

```
Perception → Normalize world state → Capabilities → Council
   → Intent lease → Execution → Tracing → Offline replay
```

Swap the adapter and `DevilutionXAdapter` becomes `GodotAdapter`. Everything from
"Normalize" rightward is reusable.

## The two repos

| | This repo (`GAP/`) | `../DevilutionX/` (first adapter) |
|---|---|---|
| What | Protocol spec + Bevy reference runtime | The real, scarred implementation |
| Lang | Rust (Bevy ECS) + Python agent | C++ engine hooks + Python council |
| Read | [GAP.md](GAP.md), this file | `ROADMAP.md`, `CLAUDE.md`, `tools/gap/` |
| Role | Where the architecture stays honest | Where it earned its scars |

The council/agent design (orchestrator, weighted voting, commitment, tracing) is
most mature in `../DevilutionX/tools/gap/`. The protocol-as-product framing lives
here.

## Run the reference runtime

Requirements: Rust 1.75+, Python 3.10+ (`pip install websockets`).

```bash
cargo run                  # Bevy ECS world, WebSocket server on ws://127.0.0.1:7777
# in another shell:
python scripts/run_agent.py   # connects, reads state, nudges the player east
```

`src/world.rs` is the toy world (a player, a skeleton, an item). `src/schema.rs`
is the wire protocol. `src/gap.rs` is the server (publish state at 30 Hz, accept
intents, coalesce). This is intentionally minimal — it's a harness for protocol
experiments, not a game.

> Note: the reference runtime still speaks the **v0.2 JSON** protocol. The
> DevilutionX adapter moved to a compact **DSL** (10× smaller, 5× fewer tokens).
> Porting the reference world to DSL is a good first task — see Lessons #2.

## Writing a new adapter

The job of an adapter is to answer three questions for the council:

1. **Perception** — encode the game's world state into the protocol (self-describing; see Lesson #3).
2. **Capabilities** — what actions exist, and what each costs/requires (engine-sourced, not hard-coded).
3. **Execution** — turn an intent into a real action *through the game's own systems*.

The single most important adapter decision: **find the least-invasive seam.** If
the game already has a multiplayer/networked-player layer, *that is your seam* —
join as another player rather than bolting on a puppet controller (Lesson #1).

---

## Lessons learned (the scars)

These aren't Diablo lessons. They're GAP lessons — extracted from experience, not
designed up front. Each one corrects something we originally got wrong.

### 1. The agent is a first-class player, not a special-cased puppet
**The original mistake:** drive a "companion slot" inside the human's game client
via a special controller. This produced the "entity control routing" blocker in
the v0.3 draft (commands executed on the wrong entity; the companion froze), and a
sprawl of `if (player == MyPlayer)` special cases.

**What actually worked:** the agent runs its **own** game client and **joins over
the network as a real second player.** No special case can then exist, because the
engine literally can't tell it apart from a human. Corollary: route every action
through the engine's normal command path (e.g. `NetSendCmd`), never via direct
state pokes — lockstep/deterministic netcode *desyncs* otherwise. *Don't make the
agent special; make it indistinguishable.*

### 2. Compact, self-describing perception beats verbose JSON
JSON state was 1–2 KB and 400–600 LLM tokens. A line-oriented **DSL**
(`T=123 F=2 ME=34,18,72,33 M=12@38,16,55,1`) cut that ~10× in bytes and ~5× in
tokens, and is trivial to diff and grep. Verbose perception is a tax you pay every
single tick.

### 3. Make perception self-describing; never duplicate game metadata
Our worst bugs came from a **parallel copy** of game tables in the agent (a
hard-coded `FIREBOLT=2` that was actually Healing → the agent "cast" the wrong
thing for weeks). Fix: the **engine owns the metadata; the perception layer
publishes it.** The agent reads a self-describing spell menu (`id, name, mana,
flags`), item fits/durability, a hazard layer — all engine-sourced. The agent does
*tactics*, never bookkeeping the game already knows.

### 4. Keep deterministic logic deterministic; LLMs are advisors, not simulators
Kiting math, threshold checks, pathing, "is this tile on fire" — all deterministic
code. The LLM picks targets, handles chat, makes fuzzy strategic calls. Asking an
LLM to *simulate* the game (compute damage, predict positions) is slow and wrong.
Layer it: **C++/engine executes, Python decides tactics, LLM advises strategy.**

### 5. Re-plan when the world meaningfully changes — not every tick
> "The skeleton is the cache-bust."

Walking to the shop is the *execution* of a decision, not a decision. Re-running
the whole council every 600 ms while nothing changed is pure churn (and burns
redundant LLM calls). The fix is an **intent lease**: the council commits to a
goal, and only reconvenes on an *invalidation event* (enemy appears, target dies,
HP threshold crossed, player command, goal complete). It's an invalidation model,
not a timer. Someone writing a Godot adapter should read this as: *don't re-plan
every frame; re-plan when the world meaningfully changes.*

### 6. Survival reflexes are never leased, and always preempt
Layer the council by priority tier. The top tier (take-damage avoidance, critical
heal, the player talking to you) is **always evaluated and can break any lease.**
Only goals (sell, explore, follow, upgrade) are leaseable. A lease is a held
stance; survival is the thing that's allowed to interrupt it. Get this carve-out
in from day one or a missed invalidation turns "annoying churn" into "walked into
the fire."

### 7. Add observability before intelligence; record decisions to replay offline
We almost tuned agent weights by replaying level 1 a thousand times by hand.
Instead: **log every decision as JSONL** — the full recommendation set, scores,
commitment state, winner, and the raw perception line. Now the council replays
*offline*: regression tests, weight tuning, behavior/profile tests, and a flight
recorder for "why did it do that at tick 14892." **Trace before tune.** This is
core infrastructure, not a debug print — version the schema; it's a contract.

### 8. Introduce formal modeling when coordination emerges, not before
A spec written from intuition rots while the protocol is still moving. Wait until
multiple agents actually contend (loot, portals, stances), then write a *small*,
scoped TLA+ spec per coordination feature — ideally grounded in pathologies you
*mined from the traces* (Lesson #7), not invented. Formal verification is for the
protocol once it stabilizes, not the framework while it's still being discovered.

### 9. The engine owns outcomes; the council owns decisions
A clean boundary worth stating out loud. The agent system is responsible for
*deciding well*; whether a decision *worked* (the MV hit a wall, the cast whiffed)
is the engine's domain. Tests and traces cover decisions; outcomes need live play
or a sim. Don't let the two blur — it's what keeps "test the AI" an honest claim.

---

## Where to go next

- **Protocol:** [GAP.md](GAP.md) — the spec. Sections marked ⚠️ SUPERSEDED are the
  v0.3 assumptions the lessons above correct; read them as history.
- **Reference runtime:** `src/` here — port it from JSON to DSL (Lesson #2) and add
  a trace log (Lesson #7) as the next two experiments.
- **The mature council:** `../DevilutionX/tools/gap/` + its `ROADMAP.md` — the
  intent-lease and decision-tracing designs live there in detail.
