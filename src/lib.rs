//! actor-supervisor — supervisor strategies for actor fault tolerance.

/// Supervision strategy applied when a child actor fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorStrategy {
    Restart,
    Stop,
    Resume,
    Escalate,
}

/// Directive returned by a supervisor when handling a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorDirective {
    Restart,
    Stop,
    Resume,
    Escalate,
}

/// Default supervisor that applies a fixed strategy.
#[derive(Debug, Clone, Copy)]
pub struct FixedSupervisor {
    strategy: SupervisorStrategy,
    max_restarts: usize,
}

impl FixedSupervisor {
    pub fn new(strategy: SupervisorStrategy) -> Self {
        Self { strategy, max_restarts: 3 }
    }

    pub fn with_max_restarts(mut self, n: usize) -> Self {
        self.max_restarts = n;
        self
    }

    pub fn decide(&self, restart_count: usize) -> SupervisorDirective {
        if restart_count >= self.max_restarts {
            return SupervisorDirective::Escalate;
        }
        match self.strategy {
            SupervisorStrategy::Restart => SupervisorDirective::Restart,
            SupervisorStrategy::Stop => SupervisorDirective::Stop,
            SupervisorStrategy::Resume => SupervisorDirective::Resume,
            SupervisorStrategy::Escalate => SupervisorDirective::Escalate,
        }
    }
}

impl Default for FixedSupervisor {
    fn default() -> Self {
        Self::new(SupervisorStrategy::Restart)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restart_then_escalate() {
        let sup = FixedSupervisor::default();
        assert_eq!(sup.decide(0), SupervisorDirective::Restart);
        assert_eq!(sup.decide(3), SupervisorDirective::Escalate);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
