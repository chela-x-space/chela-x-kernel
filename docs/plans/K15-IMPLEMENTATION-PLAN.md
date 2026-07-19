# K15 Implementation Plan

## Status
Planning Complete

## Last Updated
2026-07-19

## Proposed Official Title
`K15 External Intake Trust Boundary`

## Current Milestone State

- `K15 PLANNING: COMPLETE`
- `K15 ARCHITECTURE REVIEW: PENDING HUMAN REVIEW`
- `K15 IMPLEMENTATION AUTHORIZATION: NOT AUTHORIZED`
- `K15 IMPLEMENTATION: NOT STARTED`
- `ADR REQUIRED: YES`

## Repository Evidence Reviewed

- `README.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `docs/API.md`
- `docs/API-FREEZE.md`
- `docs/IMPLEMENTATION-PLAN.md`
- `docs/TRACEABILITY.md`
- `docs/VALIDATION.md`
- `docs/plans/K14-IMPLEMENTATION-PLAN.md`
- `docs/backlog/K14-BACKLOG.md`
- `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md`
- `docs/kernel-architecture/11-api-gateway-architecture.md`
- `docs/kernel-architecture/13-data-flow.md`
- `docs/kernel-architecture/15-roadmap.md`
- `docs/kernel-architecture/16-traceability.md`
- `crates/kernel-adapter/src/lib.rs`
- `crates/kernel-service/src/lib.rs`

## Problem Statement

K14 closes the adapter and service contract separation above frozen
`kernel-service`, but the repository still lacks an approved
post-K14 boundary for untrusted external-caller trust semantics entering
`kernel-adapter` without selecting transport, runtime, hosting,
deployment, or infrastructure technology.

The frozen K1-K11 architecture still records the API Gateway as the
canonical external integration boundary, while accepted `ADR-0003`
authorizes only the external-adapter contract boundary above
`kernel-service`. The next smallest additive step therefore is not
transport or execution. It is planning the trust-intake contract space
around K14 without implementing it.

## Objective

Define the smallest additive, deterministic, technology-neutral planning
baseline for external-caller trust semantics around frozen K14 while:

- preserving K1-K14 frozen APIs
- avoiding transport and runtime selection
- avoiding execution-layer duplication with frozen K8
- preventing lower-layer bypass
- enabling future deterministic contract validation

## Architectural Position

```text
External Caller / External System
                ↓
K15 External Intake Trust Boundary
                ↓
K14 External Adapter Boundary
                ↓
K13 Service Integration
                ↓
K12 Application Integration
                ↓
K11 Studio Integration
                ↓
K10 API Gateway
                ↓
kernel-domain
```

K15 planning is boundary planning only. It does not authorize a new
crate, source code, runtime, or infrastructure.

## Dependencies

- frozen K1-K14 public APIs
- accepted `ADR-0003`
- frozen data-flow and traceability chapters
- existing `kernel-adapter -> kernel-service` dependency direction

## Allowed Dependencies

If future implementation is ever authorized, planning assumes:

- primary production dependency remains `kernel-adapter -> kernel-service`
- K15 concepts, if approved, should compose frozen K14 contracts rather
  than bypass them
- documentation, validation, and static-audit evidence may expand
  additively

## Forbidden Dependencies

- direct production dependency from a future K15 concern to
  `kernel-application`
- direct production dependency from a future K15 concern to
  `kernel-studio`
- direct production dependency from a future K15 concern to
  `kernel-gateway`
- direct production dependency from a future K15 concern to
  `kernel-domain`
- any reverse dependency from frozen lower crates to K15 concerns
- any implementation that bypasses `kernel-adapter` or `kernel-service`

## Scope

- repository-backed planning only
- post-K14 trust-intake gap analysis
- deterministic contract categories only
- validation-matrix planning only
- ADR requirement assessment
- compatibility and architecture-risk recording

## Non-Goals

- no implementation
- no new crate
- no source code
- no Cargo change
- no dependency change
- no runtime
- no infrastructure
- no HTTP, REST, gRPC, WebSocket, GraphQL, queue, broker, or filesystem behavior
- no external API client
- no persistence or deployment architecture
- no duplication of K3 authorization authority
- no duplication of K8 execution authority

## Proposed Contract Categories

- external caller identity reference
- intake trust classification reference
- adapter intake compatibility reference
- adapter intake request context
- intake admission decision
- intake rejection reason
- intake audit continuity reference
- adapter-to-service correlation continuity binding

## Planned Requirements

| Requirement | Planned title | Repository evidence | Status |
| --- | --- | --- | --- |
| `K15-001` | Official K15 milestone definition | frozen roadmap stops at K11; post-K11 milestones require repository governance and ADR evidence | `PLANNED` |
| `K15-002` | External-caller trust boundary definition around K14 | K14 closes adapter/service separation but does not define caller trust semantics | `PLANNED` |
| `K15-003` | Frozen K14 boundary preservation | `kernel-adapter -> kernel-service` is authoritative and must not be bypassed | `PLANNED` |
| `K15-004` | Deterministic intake identity and trust continuity | K14 already preserves adapter/service continuity and identity separation | `PLANNED` |
| `K15-005` | Technology-neutral intake admission contracts | frozen architecture forbids transport/framework selection without ADR | `PLANNED` |
| `K15-006` | Safe rejection and audit continuity | K10-K14 emphasize auditability, traceability, and deterministic rejection paths | `PLANNED` |
| `K15-007` | Compatibility preservation for K1-K14 | K14 is frozen for K15 consumption and additive compatibility must remain preserved | `PLANNED` |
| `K15-008` | Static dependency and no-bypass audit | K13 and K14 closure evidence requires dependency-direction preservation | `PLANNED` |
| `K15-009` | Native-validation planning for future host verification | K14 closure records authoritative host validation as required acceptance evidence | `PLANNED` |
| `K15-010` | Governance and ADR gate before implementation | new trust-boundary authority requires explicit human approval | `PLANNED` |

## Planned Test Groups

- intake identity validation tests
- trust-classification validation tests
- continuity and separation tests
- safe rejection and audit-preservation tests
- compatibility and no-bypass conformance tests
- static dependency audit tests

## Validation Matrix

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --doc --workspace`
- `cargo doc --workspace --no-deps`
- `git diff --check`
- `git status --short`
- future host-native `cargo test` authority if implementation is later authorized

## Compatibility Constraints

- K1-K13 frozen public APIs remain unchanged
- K14 public API remains frozen for K15 consumption
- K15 must be additive only
- no K14 bypass
- no K13 bypass
- no reverse dependency from frozen lower crates

## Architecture Risks

- introducing a new trust boundary changes architectural authority
- transport or hosting concerns could leak into planning if the boundary
  is not constrained tightly
- a future K15 implementation could accidentally duplicate K10 gateway
  or K3 authorization semantics
- a future K15 implementation could accidentally collide with frozen K8
  execution concepts

## Security Risks

- caller trust semantics could be underspecified and weaken boundary
  validation
- identity continuity could be weakened if trust intake and adapter
  request identities are conflated
- audit continuity could be lost if rejection paths are not explicit

## Technology-Neutrality Assessment

K15 planning remains technology-neutral because it does not choose:

- transport protocol
- serialization format
- runtime
- hosting model
- persistence model
- deployment topology
- infrastructure provider

## ADR Requirement Assessment

`ADR REQUIRED: YES`

Reason:

- K15 planning proposes a new trust boundary around frozen K14
- new trust-boundary authority is explicitly listed as ADR-gated
- frozen K1-K11 architecture does not define post-K14 authority
- accepted `ADR-0003` does not authorize this next boundary

## Implementation Authorization State

```text
K15 PLANNING:
COMPLETE

K15 ARCHITECTURE REVIEW:
PENDING HUMAN REVIEW

K15 IMPLEMENTATION AUTHORIZATION:
NOT AUTHORIZED

K15 IMPLEMENTATION:
NOT STARTED
```
