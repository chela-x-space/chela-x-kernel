# K14 Implementation Plan

## Status
Planning Complete

## Last Updated
2026-07-19

## Proposed Official Title
`K14 External Adapter Boundary`

## Current Milestone State

- `K14 PLANNING: COMPLETE`
- `K14 ARCHITECTURE REVIEW: PASSED`
- `K14 IMPLEMENTATION AUTHORIZATION: APPROVED`
- `K14 IMPLEMENTATION: AUTHORIZED`
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

## Proposed Crate Boundary

Proposed additive workspace crate:

`crates/kernel-adapter`

Repository-authoritative status:

- proposed only
- approved for creation within ADR-0003
- authorized for implementation within the accepted K14 boundary

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
- define proposed adapter contracts only
- define compile, documentation, static audit, and future native-test planning
- identify ADR gates for any future host, transport, or runtime work

## Explicit Non-Goals

- K14 implementation
- production Rust source
- tests
- Cargo changes
- HTTP, REST, WebSocket, SSE, gRPC, or IPC hosting
- network listener ownership
- async runtime ownership
- process hosting or supervision
- persistence, cache, or session storage
- scheduler, worker, poller, or background synchronization
- deployment, ingress, TLS, topology, or observability infrastructure
- K1-K13 API changes

## Proposed Public Contracts Only

- `AdapterApiVersion`
- `AdapterIdentity`
- `AdapterIdentityKind`
- `AdapterCapabilityReference`
- `AdapterCapabilityDeclaration`
- `AdapterRequestId`
- `AdapterRequestContext`
- `AdapterCommandIntent`
- `AdapterQueryIntent`
- `AdapterResponseKind`
- `AdapterResponseStatusReference`
- `AdapterResponseEnvelope`
- `AdapterError`
- `AdapterErrorCode`
- `AdapterResult`
- `AdapterDependencyCompatibilityReference`
- `AdapterStatusSnapshot`
- `AdapterValidationStatus`

## Planned Requirements

| Requirement | Source evidence | Intended behavior | Dependency | Validation approach | Compatibility constraint | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `K14-001` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | define the next additive external-adapter title and role above K13 only | human architecture authority | architecture review | K1-K13 remain frozen | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-002` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md`, `docs/kernel-architecture/13-data-flow.md` | K14 consumes K13 and never bypasses K13, K12, K11, or K10 | frozen K13 boundary | static dependency audit | no lower-layer bypass | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-003` | `README.md`, `ARCHITECTURE.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | K14 remains additive, deterministic, technology-neutral, transport-neutral, and infrastructure-free | architecture freeze | static architecture audit | no runtime or infrastructure introduction | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-004` | `docs/TRACEABILITY.md`, `docs/kernel-architecture/16-traceability.md` | K14 preserves repository traceability without fabricated CES identifiers | inherited traceability baseline | documentation review | `PARTIAL / INHERITED` only unless approved otherwise | `PLANNED` |
| `K14-005` | `docs/API.md`, `docs/API-FREEZE.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter identity and capability contracts preserve frozen K13 service meaning only | frozen K13 service contracts | contract planning | no second identity or capability model | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-006` | `docs/API.md`, `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter command and query intent remain coordinated through K13 only | frozen K13 command/query semantics | failure-path planning | no parallel command/query semantics | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-007` | `docs/API.md`, `docs/VALIDATION.md`, `docs/ADR-0003-K14-EXTERNAL-ADAPTER-BOUNDARY.md` | adapter request and response continuity preserve service identity, scope, correlation, and audit evidence | frozen K13 request and response evidence | contract planning | no K13 semantic drift | `AUTHORIZED FOR IMPLEMENTATION` |
| `K14-008` | `docs/VALIDATION.md` | K14 defines only compile, static audit, documentation, and future native-validation gates in planning | repository validation baseline | documentation review | no implementation implied by planning | `PLANNED` |
| `K14-009` | `docs/ADR-0002-K13-SERVICE-INTEGRATION-BOUNDARY.md` | any host, transport, runtime, or deployment concern remains outside K14 planning until separately approved | later ADRs | governance review | no infrastructure in K14 planning | `PLANNED` |
| `K14-010` | `README.md`, `docs/IMPLEMENTATION-PLAN.md` | K14 must preserve K1-K13 compatibility with no reverse dependency from frozen lower-layer crates | frozen K1-K13 APIs | static dependency audit | no reverse dependency | `BLOCKED PENDING ADR` |

## Planned Test Groups

- contract identity and version tests
- adapter capability admission tests
- command and query intent separation tests
- request and response continuity tests
- compatibility and static audit tests
- failure-path and rejection-precedence tests

## Planned Validation Matrix

- compile validation: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo check --workspace --all-features --all-targets`
- lint validation: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- documentation validation: `cargo doc --workspace --no-deps`, `cargo test --doc --workspace`, `git diff --check`
- static dependency audit: confirm no reverse dependency from frozen crates into proposed K14
- static architecture audit: confirm no runtime, transport, persistence, scheduler, networking, hosting, or infrastructure concerns are introduced in planning
- native host tests: not required for planning-only closure and not applicable until implementation exists

## Risks

- the roadmap baseline does not yet define a post-K13 adapter layer, so title, crate, and dependency direction require architecture authority
- adapter-planning scope can silently become transport or host planning if listener, protocol, or deployment concerns are allowed into the milestone
- direct lower-layer dependency requests would weaken the accepted K13 boundary

## ADR Requirement Assessment

`ADR REQUIRED: YES`

Reason:

- K14 would introduce a new layer above the accepted K13 boundary
- the frozen roadmap does not define a K14 milestone title or crate boundary
- any adapter-facing boundary risks selecting transport, host, or deployment semantics unless it is explicitly constrained first

## Human Decisions Required

- implementation must remain within accepted `ADR-0003`
- implementation must preserve `kernel-adapter -> kernel-service`
- implementation must remain contract-only and infrastructure-free
- any transport, host, runtime, or deployment expansion requires a later ADR

## Governance Recommendation

- `K14 PLANNING: COMPLETE`
- `K14 ARCHITECTURE REVIEW: PASSED`
- `ADR REQUIRED: YES`
- `K14 IMPLEMENTATION AUTHORIZATION: APPROVED`
- `K14 IMPLEMENTATION: AUTHORIZED`
