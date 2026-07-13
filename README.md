# CHELA-X Kernel

## Status
Draft

## Version
0.2.1

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
`K1.1 Domain API Frozen For K2 Consumption`

## Constraints
- Architecture is frozen.
- No redesign may occur without an approved ADR.
- K1 domain model is implemented.
- K1.1 validation is `PASS WITH BLOCKERS`.
- Runtime execution is not implemented.
- Domain API baseline is frozen for K2 consumers.
- Runtime execution is not implemented.
- No business logic, persistence, networking, or workflow execution is introduced in K1.
- No frozen upstream repository may be modified by this repository.

## Domain Scope
- Stable identifiers
- Enterprise ownership paths
- Immutable identity primitives
- Lifecycle state types
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
- [docs/VALIDATION.md](./docs/VALIDATION.md)
