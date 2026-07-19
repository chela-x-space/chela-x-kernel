# K13 Implementation Plan

## Status
Planning Complete

## Last Updated
2026-07-19

## Exact K13 Title
`K13 Service Integration`

## Current Milestone State

- `K13 PLANNING: COMPLETE`
- `K13 ARCHITECTURE REVIEW: BLOCKED PENDING ADR`
- `K13 IMPLEMENTATION AUTHORIZATION: BLOCKED`
- `K13 IMPLEMENTATION: NOT STARTED`
- `ADR REQUIRED: YES`

## Purpose
Record the smallest architecture-preserving K13 planning baseline above
frozen `K12 Application Integration` without introducing runtime,
transport, persistence, scheduler, or other infrastructure.

## Repository Evidence Reviewed
- `README.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `docs/IMPLEMENTATION-PLAN.md`
- `docs/TRACEABILITY.md`
- `docs/VALIDATION.md`
- `docs/plans/K12-IMPLEMENTATION-PLAN.md`
- `docs/backlog/K12-BACKLOG.md`
- `docs/ADR-0001-K12-APPLICATION-INTEGRATION-BOUNDARY.md`
- `docs/kernel-architecture/12-studio-integration-architecture.md`
- `docs/kernel-architecture/13-data-flow.md`
- `docs/kernel-architecture/15-roadmap.md`
- `docs/kernel-architecture/16-traceability.md`

## Authoritative Repository Finding
The frozen architecture baseline defines approved milestones through
`K11 Studio Integration`.

`K12 Application Integration` exists only because `ADR-0001` explicitly
created that architectural boundary.

No repository-authoritative document currently defines:

- an official K13 title
- a K13 crate boundary
- a K13 architectural role
- a K13 dependency direction above `kernel-application`

## Problem Statement
Future service-facing adapters need a governed coordination boundary
above frozen `kernel-application` without:

- bypassing K12
- bypassing K11
- bypassing K10
- selecting transport or runtime architecture
- introducing persistence, scheduling, or service infrastructure

The repository does not currently authorize that boundary.

## Proposed K13 Characterization
Proposed exact title:

`K13 Service Integration`

Proposed architectural role:

`Technology-neutral service coordination boundary above K12 Application Integration`

Proposed dependency position:

```text
External Service Adapter
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

## Planning Scope
- define the smallest service-facing coordination boundary above K12
- preserve frozen K1-K12 public APIs unchanged
- preserve K12 as the only approved application coordination boundary
- preserve K11 Studio coordination and K10 Gateway governance
- identify K13 trust-boundary obligations
- define proposed contracts only
- define validation and static audit gates
- identify ADR requirements before implementation

## Out Of Scope
- K13 implementation
- production Rust source
- tests
- Cargo changes
- transport protocol selection
- HTTP, REST, WebSocket, SSE, gRPC, or IPC hosting
- async runtime ownership
- process hosting or supervision
- persistence, cache, or session storage
- scheduler, worker, poller, or background synchronization
- authentication-provider integration
- deployment, ingress, TLS, or topology decisions
- K1-K12 API changes

## Frozen Dependencies
- K10 API Gateway contracts
- K11 Studio Integration contracts
- K12 Application Integration contracts

K13 may consume lower frozen layers only through K12 unless a later ADR
proves an exceptional dependency is necessary and non-bypassing.

## Proposed Crate Boundary
Proposed additive workspace crate:

`crates/kernel-service`

Repository-authoritative status:

- proposed only
- not approved for creation
- blocked pending ADR

## Proposed Module Boundary
- `service.rs`
- `service_identity.rs`
- `service_context.rs`
- `service_session.rs`
- `service_view.rs`
- `service_command.rs`
- `service_query.rs`
- `service_response.rs`
- `service_error.rs`
- `service_capability.rs`
- `service_status.rs`
- `service_validation.rs`

These module names are planning placeholders only.

## Proposed Contracts Only
- `ServiceApiVersion`
- `ServiceIdentity`
- `ServiceIdentityKind`
- `ServiceCapabilityReference`
- `ServiceCapabilityDeclaration`
- `ServiceRequestId`
- `ServiceAuditReference`
- `ServiceRequestContext`
- `ServiceSessionReference`
- `ServiceSessionStatusReference`
- `ServiceViewIntent`
- `ServiceCommandIntent`
- `ServiceQueryIntent`
- `ServiceRequestEnvelope`
- `ServiceResponseKind`
- `ServiceResponseStatusReference`
- `ServiceResponsePayload`
- `ServiceResponseEnvelope`
- `ServiceError`
- `ServiceErrorCode`
- `ServiceResult`
- `ServiceDependencyCompatibilityReference`
- `ServiceStatusSnapshot`
- `ServiceValidationStatus`

## Proposed Responsibilities Only
K13 may define additive, deterministic, technology-neutral contracts
for:

- service identity
- service request context
- service session reference without storage
- service capability declaration
- service navigation, view, command, and query intent
- service request and response envelopes
- service error envelopes
- service correlation continuity
- service scope continuity
- service audit continuity
- service status snapshots
- centralized service integration validation

## Explicit Non-Responsibilities
K13 must not own:

- HTTP server
- REST router
- WebSocket server
- SSE server
- gRPC server
- IPC server
- TCP listener
- async runtime
- process host
- process supervisor
- scheduler
- worker
- poller
- database
- filesystem persistence
- cache
- session store
- identity provider
- authentication provider
- authorization engine
- frontend framework
- browser runtime
- desktop runtime
- static asset hosting
- deployment topology
- TLS termination
- reverse proxy
- observability backend

## Trust Boundary And Security Constraints
K13 would be a service-facing coordination boundary.

It must preserve, not replace:

- K12 application identity semantics
- K12 request and response continuity
- K11 Studio view and intent separation
- K10 authentication and authorization evidence
- K10 request validation and error semantics
- K1-K12 scope, correlation, and audit continuity

K13 must not create:

- a second application model
- a second service-authorization model
- a second scope model
- a second correlation model
- a second audit model
- parallel command or query semantics

## Risk Assessment
High architectural risk:

- naming or approving K13 without ADR would expand the frozen milestone
  model without authority
- allowing K13 to depend directly on `kernel-gateway` or `kernel-domain`
  would risk bypassing K12 and K11

Medium design risk:

- service contracts can silently become transport contracts if routing,
  protocol, or listener concerns are mixed into the milestone
- service session references can silently become session storage or login
  state if not bounded explicitly

Low implementation risk after ADR:

- a pure contract-only crate above K12 can remain additive and
  infrastructure-free if it is explicitly constrained by ADR

## Validation Plan
- compile validation: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo check --workspace --all-features --all-targets`
- lint validation: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- documentation validation: `cargo doc --workspace --no-deps`, `cargo test --doc --workspace`, `git diff --check`, `git status --short`
- static dependency audit: confirm no reverse dependency from frozen crates into proposed K13
- static architecture audit: confirm no runtime, transport, persistence, scheduler, networking, or infrastructure concerns are introduced in planning
- native host tests: not applicable until K13 implementation exists

## Requirements Matrix
| Requirement | Source evidence | Intended behavior | Dependency | Validation approach | Compatibility constraint | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `K13-001` | `docs/ADR-0001-K12-APPLICATION-INTEGRATION-BOUNDARY.md`, `docs/kernel-architecture/15-roadmap.md` | define an explicit K13 title and role above K12 only through approved architecture authority | human architecture authority | architecture review | K1-K12 remain frozen | `BLOCKED PENDING ADR` |
| `K13-002` | `docs/plans/K12-IMPLEMENTATION-PLAN.md`, `docs/kernel-architecture/13-data-flow.md` | K13 consumes K12 and never bypasses K12, K11, or K10 | frozen K12 boundary | static dependency audit | no lower-layer bypass | `PLANNED` |
| `K13-003` | `README.md`, `ARCHITECTURE.md` | K13 remains additive, deterministic, technology-neutral, transport-neutral, and infrastructure-free | architecture freeze | static architecture audit | no runtime or infrastructure introduction | `PLANNED` |
| `K13-004` | `docs/kernel-architecture/16-traceability.md`, `docs/TRACEABILITY.md` | K13 preserves repository traceability without fabricating CES identifiers | inherited traceability baseline | documentation review | `PARTIAL / INHERITED` only unless approved otherwise | `PLANNED` |
| `K13-005` | `docs/kernel-architecture/11-api-gateway-architecture.md`, `docs/kernel-architecture/12-studio-integration-architecture.md` | K13 preserves K10 and K11 semantics through K12 coordination only | frozen K10-K12 semantics | contract and failure-path planning | no reinterpretation of lower-layer meaning | `PLANNED` |
| `K13-006` | architecture freeze and absence of K13 in frozen baseline | K13 crate boundary and dependency direction require architecture approval before implementation | approved ADR | architecture review | no `kernel-domain`, `kernel-gateway`, `kernel-studio`, or `kernel-application` reverse dependency | `BLOCKED PENDING ADR` |
| `K13-007` | repository validation conventions | K13 planning defines compile, static audit, and future native validation gates | repository validation baseline | documentation review | no implementation implied by planning | `PLANNED` |
| `K13-008` | future runtime and transport concerns remain separate from K13 planning | transport, runtime, persistence, scheduler, and networking concerns remain blocked | later ADRs | governance review | no infrastructure in K13 planning | `REQUIRES LATER ADR` |

## Governance Recommendation
- `K13 PLANNING: COMPLETE`
- `K13 ARCHITECTURE REVIEW: BLOCKED PENDING ADR`
- `ADR REQUIRED: YES`
- `K13 IMPLEMENTATION AUTHORIZATION: BLOCKED`
- `K13 IMPLEMENTATION: NOT STARTED`

## ADR Recommendation
`REQUIRED`

The repository needs a new ADR before K13 implementation because the
frozen baseline does not define:

- the official K13 milestone title
- the K13 architectural role
- the K13 crate boundary
- the K13 dependency direction above K12

One repository-local ADR is sufficient for the initial K13 contract-only
boundary. Additional ADRs would still be required later if K13 selects:

- a concrete transport
- a runtime owner
- persistence or session infrastructure
- service hosting or deployment architecture
