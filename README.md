# CHELA-X Kernel

## Status
Draft

## Version
0.2.3

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

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

## Current Status
`K2 State And Lifecycle Implemented; Final PASS Blocked By Missing Native Linker`

## Constraints
- Architecture is frozen.
- No redesign may occur without an approved ADR.
- K1 domain model is implemented.
- K1.1 validation is `PASS`.
- Domain API freeze status is `FROZEN FOR K2 CONSUMPTION`.
- Ready for K2 is `YES`.
- K2 implementation is additive in `crates/kernel-domain/src/state.rs`.
- K2 validation is blocked on `cargo test --workspace --all-targets` because `cc`, `gcc`, and `clang` are not installed in this environment.
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

## References
- [AGENTS.md](./AGENTS.md)
- [ENGINEERING.md](./ENGINEERING.md)
- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [docs/BASELINE.md](./docs/BASELINE.md)
- [docs/TRACEABILITY.md](./docs/TRACEABILITY.md)
- [docs/IMPLEMENTATION-PLAN.md](./docs/IMPLEMENTATION-PLAN.md)
- [docs/K2-STATE-LIFECYCLE.md](./docs/K2-STATE-LIFECYCLE.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
