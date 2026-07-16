# CHELA-X Kernel

## Status
Implementation (K5.1 Complete)

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-16

## Applies To
CHELA-X Kernel repository baseline, bootstrap, and future implementation work.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose
CHELA-X Kernel is the governed implementation repository for the kernel layer that sits between CHELA-X CES and future CHELA-X Runtime.

## Dependency Position
AI Engineering OS -> CHELA-X CES -> CHELA-X Kernel -> CHELA-X Runtime -> CHELA-X SDK -> CHELA-X Media

## Baseline
- AI Engineering OS v1.0 Freeze defines global engineering, workflow, decision, architecture, and metadata rules.
- CHELA-X CES Book 0 RC1 defines the canonical enterprise requirements consumed by Kernel.
- CHELA-X Program defines product dependency direction and portfolio ordering.
- CHELA-X Library provides the read-only retrieval index used for traceability and verification.

## Current Kernel Status

| Milestone | Status |
|-----------|--------|
| K1 Domain Foundation | PASS |
| K2 State Lifecycle | PASS |
| K3 Authorization Foundation | PASS |
| K4 Runtime Supervision | PASS |
| K5.1 Canonical Event Envelope | PASS |
| K5.2 Validation | PASS |
| K5.3 Event Streams | NEXT |
| K5.4 Replay | NOT STARTED |
| K6 Runtime Event Bus | NOT STARTED |

Canonical host validation:

- **304 passed**
- **0 failed**

## Current Status
`K5.1 Canonical Enterprise Event Envelope Implemented`

## Constraints
- Architecture is frozen.
- No redesign may occur without an approved ADR.
- K1 domain model is implemented.
- K1.1 validation is `PASS`.
- Domain API freeze status is `FROZEN FOR K2 CONSUMPTION`.
- Ready for K2 is `YES`.
- Ready for K3 is `YES`.
- K2 implementation is additive in `crates/kernel-domain/src/state.rs`.
- K2 implementation is complete.
- K2 architecture review passed.
- K2 canonical host validation passed.
- Codex sandbox linker isolation is not a project blocker.
- K3 implementation is additive in `crates/kernel-domain/src/enforcement.rs`.
- K3 implementation is complete.
- K3 canonical host validation passed with `108 passed`, `0 failed`, `0 ignored`.
- K4.1 implementation is additive in `crates/kernel-domain/src/runtime.rs`.
- K4.1 implementation is complete.
- K4.1 canonical host validation passed with `108 passed`, `0 failed`, `0 ignored`.
- K4.2 implementation is additive in `crates/kernel-domain/src/runtime.rs`.
- K4.2 implementation is complete in source and awaits canonical host validation because native unit-test linking still requires a host with `cc` in this environment.
- K4.3 has not started.
- K5.1 implementation is additive in `crates/kernel-domain/src/event.rs`.
- K5.1 implementation is complete.
- K5.1 canonical host validation passed with `236 passed`, `0 failed`.
- K5.2 validation implementation is complete.
- K5.2 canonical host validation passed with `304 passed`, `0 failed`.
- K5.3 Event Streams is next.
- Runtime execution is not implemented.
- Domain API baseline is frozen for K2 consumers.
- No business logic, persistence, networking, or workflow execution is introduced in K1.
- No workflow execution, persistence, networking, or runtime orchestration is introduced in K2.
- No frozen upstream repository may be modified by this repository.

## Domain Scope
- Stable identifiers
- Enterprise ownership paths
- Immutable identity primitives
- Lifecycle state types
- State snapshots, transition requests, transition outcomes, lifecycle guards, and failure codes
- Authorization request record types
- Decision record types
- Authorization, agent, delegation, policy, and workflow reference types
- Deterministic authorization enforcement inputs, traces, results, and decision construction helpers
- Deterministic runtime registry, capability indexing, heartbeat, freshness, lease, presence, runtime-health, runtime-snapshot, and supervisor primitives

## References
- [AGENTS.md](./AGENTS.md)
- [ENGINEERING.md](./ENGINEERING.md)
- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [docs/BASELINE.md](./docs/BASELINE.md)
- [docs/TRACEABILITY.md](./docs/TRACEABILITY.md)
- [docs/IMPLEMENTATION-PLAN.md](./docs/IMPLEMENTATION-PLAN.md)
- [docs/K2-STATE-LIFECYCLE.md](./docs/K2-STATE-LIFECYCLE.md)
- [docs/K3-DECISION-AUTHORIZATION.md](./docs/K3-DECISION-AUTHORIZATION.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
