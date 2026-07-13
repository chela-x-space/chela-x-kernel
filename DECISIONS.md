# DECISIONS

## Status
Draft

## Version
0.2.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
Non-trivial implementation decisions recorded during CHELA-X Kernel bootstrap.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose
Record implementation-level decisions made during K0 without creating architectural authority.

## Decision Log
- `K0-001`: Initialize Kernel as a Rust workspace only; no business logic or runtime components are introduced in bootstrap.
- `K0-002`: Preserve CES requirements through source references and traceability tables instead of duplicating specification text.
- `K0-003`: Treat missing Rust toolchain as an environment blocker to validation, not as a reason to change scope or architecture.
- `K1-001`: Replace `kernel-bootstrap` with a neutral `kernel-domain` crate to host value objects without creating runtime architecture.
- `K1-002`: Keep K1 std-only and avoid external dependencies because the current scope does not require serialization, async runtime, persistence, or networking.
- `K1-003`: Implement only strongly typed identifiers, ownership paths, immutable identity primitives, lifecycle types, request records, decision records, and authorization/delegation references.

## References
- [ENGINEERING.md](./ENGINEERING.md)
- [docs/BASELINE.md](./docs/BASELINE.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
