# Actor Supervisor

**Actor Supervisor** is a Rust library implementing supervision strategies for actor fault tolerance — restart, stop, resume, and escalate — with configurable max-restart thresholds for cascading failure prevention.

## Why It Matters

In distributed and concurrent systems, failures are inevitable: network partitions, resource exhaustion, and logic errors crash individual actors. The supervision pattern, pioneered in Erlang/OTP and formalized in Akka, provides structured failure handling: when a child actor fails, its parent supervisor decides what to do based on a declared strategy. This transforms chaotic failure cascades into predictable, recoverable system behavior. Without supervision, a single actor panic can bring down an entire system; with it, failures are contained and handled at the appropriate level of the actor hierarchy.

## How It Works

The supervisor implements a fixed strategy with a restart budget. When a child fails, `decide(restart_count)` returns a directive:

**Strategies:**

| Strategy | Behavior | Use Case |
|----------|----------|----------|
| Restart | Kill and re-create the actor | Transient failures (bad state) |
| Resume | Continue processing next message | Benign failures (bad input) |
| Stop | Permanently terminate the actor | Unrecoverable failures |
| Escalate | Propagate to parent supervisor | Beyond this supervisor's capacity |

**Max-restart circuit breaker:**
The supervisor tracks how many times a child has restarted. When `restart_count >= max_restarts` (default 3), the directive becomes Escalate regardless of the configured strategy. This prevents infinite restart loops — a critical safety mechanism.

```
decide(restart_count):
  if restart_count >= max_restarts:
    return Escalate          // circuit breaker
  match strategy:
    Restart → Restart
    Stop → Stop
    Resume → Resume
    Escalate → Escalate
```

This implements a **One-For-One** supervision semantics: only the failed child is affected. More sophisticated strategies like All-For-One (restart all siblings) and Rest-For-One (restart the failed child and all started after it) build on this foundation.

The max-restarts threshold of 3 follows Erlang/OTP's default `maxR` in child specifications, providing a practical balance between fault tolerance and failure containment.

## Quick Start

```rust
fn main() {
    let sup = FixedSupervisor::new(SupervisorStrategy::Restart);
    assert_eq!(sup.decide(0), SupervisorDirective::Restart);  // first failure
    assert_eq!(sup.decide(2), SupervisorDirective::Restart);  // third failure
    assert_eq!(sup.decide(3), SupervisorDirective::Escalate); // circuit breaker trips
}
```

## API

| Type/Method | Description |
|-------------|-------------|
| `SupervisorStrategy` | Enum: Restart, Stop, Resume, Escalate |
| `SupervisorDirective` | Enum: Restart, Stop, Resume, Escalate |
| `FixedSupervisor::new` | Create with a fixed strategy |
| `with_max_restarts` | Set restart threshold (default 3) |
| `decide` | `(restart_count: usize) → SupervisorDirective` |

## Architecture Notes

The Supervisor implements the **fault-tolerance layer** in the SuperInstance actor system. Within γ + η = C, supervision ensures that conservation-law violations (e.g., an actor producing avoidance ratios outside the expected distribution) trigger automatic restart rather than silent corruption. The escalation chain mirrors the fleet hierarchy: γ-layer actor → γ-supervisor → η-layer coordinator → fleet orchestrator.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

1. Armstrong, J. (2003). *Making Reliable Distributed Systems in the Presence of Software Errors*. PhD Thesis, KTH. Chapter 4: Supervision Trees.
2. Ho, T.-H. & Huey, J. (2014). "Design Patterns for Supervision in Erlang/OTP." *ACM SIGPLAN Erlang Workshop*.

## License

MIT
