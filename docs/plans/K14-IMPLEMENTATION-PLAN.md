# K14 Implementation Plan

## Status
Implementation Complete

## Last Updated
2026-07-19

## Proposed Official Title
`K14 External Adapter Boundary`

## Current Milestone State

- `K14 PLANNING: COMPLETE`
- `K14 ARCHITECTURE REVIEW: PASSED`
- `K14 IMPLEMENTATION AUTHORIZATION: APPROVED`
- `K14 IMPLEMENTATION: COMPLETE`
- `K14 COMPILE VALIDATION: PASSED`
- `K14 NATIVE VERIFICATION: BLOCKED IN CURRENT CODEX ENVIRONMENT`
- `K14 API: NOT FROZEN`
- `K14 STATUS: AWAITING HUMAN REVIEW`
- `ADR REQUIRED: YES`

## Purpose
Define the smallest additive next-layer planning baseline above frozen
`K13 Service Integration` without introducing runtime, transport,
persistence, hosting, deployment, or other infrastructure.

## Repository Evidence Reviewed

- `README.md`
- `ARCHITECTURE.md`
- `ENGINEERING.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `docs/API.md`
- `docs/API-FREEZE.md`
- `docs/IMPLEMENTATION-PLAN.md`
- `docs/TRACEABILITY.md`
- `docs/VALIDATION.md`
- `docs/plans/K12-IMPLEMENTATION-PLAN.md`
- `docs/plans/K13-IMPLEMENTATION-PLAN.md`
- `docs/backlog/K12-BACKLOG.md`
- `docs/backlog/K13-BACKLOG.md`
- `docs/ADR-0001-K12-APPLICATION-INTEGRATION-BOUNDARY.md`
- `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`
- `docs/kernel-architecture/13-data-flow.md`
- `docs/kernel-architecture/15-roadmap.md`
- `docs/kernel-architecture/16-traceability.md`

## Problem Statement

`ADR-0002` closes K13 as the technology-neutral service coordination
boundary above `kernel-application`, but it explicitly leaves any
concrete external adapter, transport, runtime, deployment, or
observability decisions outside the accepted K13 boundary.

The repository therefore needs a planning baseline for the smallest
possible layer above `kernel-service` that:

- preserves K1-K13 frozen APIs
- does not bypass `kernel-service`
- does not introduce transport or runtime ownership
- does not select hosting or deployment architecture
- remains additive and technology-neutral

## Proposed Architectural Position

```text
External System / External Adapter
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
kernel-gateway
    ↓
kernel-domain
```

## Crate Boundary

Additive workspace crate:

`crates/kernel-adapter`

Repository-authoritative status:

- created in the root workspace
- implemented within accepted `ADR-0003`
- not frozen

## Dependency Direction

Primary proposed dependency:

```text
kernel-adapter -> kernel-service
```

Allowed lower-layer dependencies:

- none approved directly in planning

Any direct dependency on `kernel-application`, `kernel-studio`,
`kernel-gateway`, or `kernel-domain` would require explicit ADR
justification because it risks bypassing the accepted K13 service
boundary.

## Scope

- define a technology-neutral external-adapter contract boundary above K13
- preserve frozen K13 service semantics unchanged
- preserve K12, K11, and K10 continuity through K13 only
- define additive adapter contracts only
- define compile, documentation, static audit, and native-test validation evidence
- identify ADR gates for any future host, transport, or runtime work

## Explicit Non-Goals

- HTTP, REST, WebSocket, SSE, gRPC, or IPC hosting
- network listener ownership
- async runtime ownership
- process hosting or supervision
- persistence, cache, or session storage
- scheduler, worker, poller, or background synchronization
- deployment, ingress, TLS, topology, or observability infrastructure
- K1-K13 API changes

## Public Contracts Implemented

- `AdapterApiVersion`
- `AdapterIdentity`
- `AdapterIdentityKind`
- `AdapterKind`
- `AdapterCapabilityReference`
- `AdapterCapabilityDeclaration`
- `AdapterRequestId`
- `AdapterRequestContext`
- `AdapterCommandIntent`
- `AdapterQueryIntent`
- `AdapterRequestEnvelope`
- `AdapterResponseKind`
- `AdapterResponseStatusReference`
- `AdapterResponseEnvelope`
- `AdapterError`
- `AdapterErrorCode`
- `AdapterResult`
- `AdapterCompatibilityReference`
- `AdapterStatusSnapshot`
- `AdapterValidationStatus`

## Planned Requirements

| Requirement | Source evidence | Intended behavior | Dependency | Validation approach | Compatibility constraint | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `K14-001` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | define the next additive external-adapter title and role above K13 only | human architecture authority | architecture review | K1-K13 remain frozen | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-002` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md`, `docs/kernel-architecture/13-data-flow.md` | K14 consumes K13 and never bypasses K13, K12, K11, or K10 | frozen K13 boundary | static dependency audit | no lower-layer bypass | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-003` | `README.md`, `ARCHITECTURE.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | K14 remains additive, deterministic, technology-neutral, transport-neutral, and infrastructure-free | architecture freeze | static architecture audit | no runtime or infrastructure introduction | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-004` | `docs/TRACEABILITY.md`, `docs/kernel-architecture/16-traceability.md` | K14 preserves repository traceability without fabricated CES identifiers | inherited traceability baseline | documentation review | `PARTIAL / INHERITED` only unless approved otherwise | `IMPLEMENTED; COMPILE VERIFIED; NATIVE VERIFICATION BLOCKED` |
| `K14-005` | `docs/API.md`, `docs/API-FREEZE.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter identity and capability contracts preserve frozen K13 service meaning only | frozen K13 service contracts | contract planning | no second identity or capability model | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-006` | `docs/API.md`, `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter command and query intent remain coordinated through K13 only | frozen K13 command/query semantics | failure-path planning | no parallel command/query semantics | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-007` | `docs/API.md`, `docs/VALIDATION.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter request and response continuity preserve service identity, scope, correlation, and audit evidence | frozen K13 request and response evidence | contract planning | no K13 semantic drift | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-008` | `docs/VALIDATION.md` | K14 defines compile, static audit, documentation, and attempted native-validation gates in implementation evidence | repository validation baseline | documentation review | no runtime or infrastructure introduced | `IMPLEMENTED; COMPILE VERIFIED; NATIVE VERIFICATION BLOCKED` |
| `K14-009` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | host, transport, runtime, persistence, and infrastructure remain outside implemented K14 | later ADRs | governance review | no infrastructure in K14 implementation | `IMPLEMENTED; COMPILE VERIFIED; NATIVE VERIFICATION BLOCKED` |
| `K14-010` | `README.md`, `docs/IMPLEMENTATION-PLAN.md` | K14 preserves K1-K13 compatibility with no reverse dependency from frozen lower-layer crates | frozen K1-K13 APIs | static dependency audit | no reverse dependency | `IMPLEMENTED; COMPILE VERIFIED; NATIVE VERIFICATION BLOCKED` |

## Test Groups

- contract identity and version tests
- adapter capability admission tests
- command and query intent separation tests
- request and response continuity tests
- compatibility and static audit tests
- failure-path and rejection-precedence tests

## Validation Matrix

- compile validation: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo check --workspace --all-features --all-targets`
- lint validation: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- documentation validation: `cargo doc --workspace --no-deps`, `cargo test --doc --workspace`, `git diff --check`
- static dependency audit: confirm no reverse dependency from frozen crates into proposed K14
- static architecture audit: confirm no runtime, transport, persistence, scheduler, networking, hosting, or infrastructure concerns are introduced in planning
- native host tests: attempted in the current Codex environment and blocked by missing linker `cc`

## Risks

- the roadmap baseline does not yet define a post-K13 adapter layer, so title, crate, and dependency direction require architecture authority
- adapter implementation scope can silently become transport or host planning if listener, protocol, or deployment concerns are allowed into the milestone
- direct lower-layer dependency requests would weaken the accepted K13 boundary

## ADR Requirement Assessment

`ADR REQUIRED: YES`

Reason:

- K14 would introduce a new layer above the accepted K13 boundary
- the frozen roadmap does not define a K14 milestone title or crate boundary
- any adapter-facing boundary risks selecting transport, host, or deployment semantics unless it is explicitly constrained first

## Human Decisions Required

- human native-verification rerun on a host with linker `cc`
- human review of K14 API before freeze
- any transport, host, runtime, or deployment expansion requires a later ADR

## Governance Recommendation

- `K14 PLANNING: COMPLETE`
- `K14 ARCHITECTURE REVIEW: PASSED`
- `ADR REQUIRED: YES`
- `K14 IMPLEMENTATION AUTHORIZATION: APPROVED`
- `K14 IMPLEMENTATION: COMPLETE`
- `K14 COMPILE VALIDATION: PASSED`
- `K14 NATIVE VERIFICATION: BLOCKED IN CURRENT CODEX ENVIRONMENT`
- `K14 API: NOT FROZEN`
- `K14 STATUS: AWAITING HUMAN REVIEW`
