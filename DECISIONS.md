# DECISIONS

## Status
Draft

## Version
0.2.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-19

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

Repository-local architectural decisions that require human approval are recorded as ADR documents under `docs/ADR-*` when introduced.

## Decision Log
- `K0-001`: Initialize Kernel as a Rust workspace only; no business logic or runtime components are introduced in bootstrap.
- `K0-002`: Preserve CES requirements through source references and traceability tables instead of duplicating specification text.
- `K0-003`: Treat missing Rust toolchain as an environment blocker to validation, not as a reason to change scope or architecture.
- `K1-001`: Replace `kernel-bootstrap` with a neutral `kernel-domain` crate to host value objects without creating runtime architecture.
- `K1-002`: Keep K1 std-only and avoid external dependencies because the current scope does not require serialization, async runtime, persistence, or networking.
- `K1-003`: Implement only strongly typed identifiers, ownership paths, immutable identity primitives, lifecycle types, request records, decision records, and authorization/delegation references.

## ADR Register
- `ADR-0001`: `K12 Application Integration Boundary` — `ACCEPTED` on `2026-07-19`; repository-local Kernel ADR numbering is independent of `CES-ADR-*` identifiers in other repositories; K12 implementation is authorized within the accepted ADR boundary.
- `ADR-0002`: `K13 Service Integration Boundary` — `ACCEPTED` on `2026-07-19`; authorizes a technology-neutral service coordination boundary above `kernel-application` while preserving K1-K12 APIs and forbidding K12 bypass or direct domain mutation.
- `ADR-0003`: `K14 External Adapter Boundary` — `ACCEPTED` on `2026-07-19`; authorizes a transport-neutral external-adapter contract boundary above `kernel-service` while preserving K1-K13 APIs and forbidding K13 bypass or direct lower-layer invocation. K14 implementation, workspace integration, compile validation, native verification, architecture conformance, and API freeze are closed on the authoritative primary host.
- `ADR-0004`: `K15 External Intake Trust Boundary` — `PROPOSED` on `2026-07-20`; proposes a transport-neutral and technology-neutral external intake trust boundary positioned before the K14 External Adapter Boundary. K15 separates claimed identity, observed source, verified identity references, trust classification, trust evidence references, correlation, audit continuity, and adapter handoff. No implementation is authorized. K15 implementation remains pending explicit Human Architecture Acceptance and Human Implementation Authorization.

## References
- [ENGINEERING.md](./ENGINEERING.md)
- [docs/BASELINE.md](./docs/BASELINE.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
