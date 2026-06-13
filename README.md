# Actor Supervisor

A **supervisor** in the actor model monitors child actors and decides what to do when they fail — resume, restart, stop, or escalate the failure.

## Why It Matters

Supervision is what makes actor systems self-healing. Instead of try/catch across async boundaries, failures bubble up to supervisors that have context about the subsystem. This is the foundation of Erlang/OTP's 'let it crash' philosophy.

## How It Works

Implements supervision strategies: OneForOne (restart just the failed child), AllForOne (restart all children), RestForOne (restart the failed child and all created after it). Each child has a configurable restart limit with backoff.

## Usage

```toml
[dependencies]
actor-supervisor = "0.1.0"
```

```rust
use actor_supervisor;

// See examples/ directory for detailed usage
```

## API

- `SupervisorStrategy` (lib.rs)
- `SupervisorDirective` (lib.rs)
- `FixedSupervisor` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
