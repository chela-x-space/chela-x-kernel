# ADR-0001: K12 Application Integration Boundary

## Status
PROPOSED

## Date
2026-07-19

## Deciders
- Chief Enterprise Architect
- Kernel Platform Team

## Contributing Role
- Implementation Engineer

## Context
The frozen CHELA-X Kernel architecture baseline currently defines approved milestones through `K11 Studio Integration`.

K11 is closed and frozen as a technology-neutral Studio contract layer above the frozen K10 API Gateway.

The repository planning baseline created in commit `f43f180` established that:

- K12 planning is authorized
- K12 implementation is not authorized
- the repository does not contain an approved K12 title
- the repository does not contain an approved K12 crate boundary
- any concrete post-K11 implementation would otherwise force unapproved architectural choices

The architecture freeze remains active.

The repository needs an architectural decision that defines what K12 is without selecting infrastructure, runtime ownership, transport protocol, frontend framework, persistence engine, authentication provider, deployment topology, or hosting platform.

## Decision
For architectural planning purposes, the proposed official K12 title is:

`K12 Application Integration`

The proposed architectural role is:

`Technology-neutral application coordination and integration boundary above K11 Studio Integration`

The proposed architectural position is:

```text
External Application or Adapter
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

The proposed first K12 implementation phase is constrained to:

- deterministic behavior
- contract-oriented behavior
- technology-neutral behavior
- transport-neutral behavior
- infrastructure-free behavior
- additive workspace evolution
- side-effect-free behavior

## Architectural Meaning
K12 is proposed as an application-facing coordination layer.

K12 is not proposed as:

- a transport layer
- a runtime host
- a dashboard backend runtime
- a frontend application
- a concrete adapter implementation
- a persistence subsystem

K12 coordinates application-facing intent and continuity semantics while preserving the frozen K10 and K11 boundaries.

## Layer Placement
K12 is proposed above `kernel-studio` and below any concrete external application or adapter.

K12 must not be referenced by frozen lower layers.

## Proposed Workspace Boundary
The proposed additive workspace crate name is:

`crates/kernel-application`

This name is proposed, not yet implemented.

The proposed dependency direction is:

```text
kernel-application
    -> kernel-studio
    -> kernel-gateway
    -> kernel-domain
```

Allowed dependencies:

- `kernel-studio`
- `kernel-gateway`
- `kernel-domain` only when required by frozen value or reference types already exposed through K10 or K11 coordination

Forbidden reverse dependencies:

```text
kernel-domain -> kernel-application
kernel-gateway -> kernel-application
kernel-studio -> kernel-application
```

Forbidden bypasses:

```text
kernel-application -> direct kernel-domain mutation
external application -> direct kernel-domain access
external application -> bypass K10
external application -> bypass K11
```

## Authorized Responsibilities After ADR Acceptance
If this ADR is accepted, K12 may define additive, technology-neutral coordination contracts for:

- application identity
- application request context
- application session reference without session storage
- application navigation intent
- application view intent
- application command intent
- application query intent
- application response envelope
- application error envelope
- application correlation continuity
- application scope continuity
- application audit continuity
- application capability declaration
- application status snapshot
- application integration validation

K12 may:

- consume frozen K11 Studio contracts
- consume frozen K10 Gateway contracts
- submit commands and queries only through frozen K11 and K10 boundaries
- wrap and coordinate frozen contracts without reinterpreting their meaning incompatibly

## Explicit Non-Responsibilities
K12 must not own:

- HTTP server
- REST routing
- WebSocket server
- SSE server
- IPC server
- TCP listener
- async runtime
- process supervisor
- background worker
- scheduler
- poller
- database
- filesystem persistence
- cache
- session store
- identity provider
- authentication provider
- authorization policy engine
- frontend framework
- browser runtime
- desktop runtime
- static asset hosting
- deployment topology
- container orchestration
- TLS termination
- reverse proxy

## Ownership Model
Runtime ownership: `NONE`

Transport ownership: `NONE`

Persistence ownership: `NONE`

Frontend ownership: `NONE`

Authentication-provider ownership: `NONE`

Deployment ownership: `NONE`

Side-effect ownership: `NONE`

## Security And Trust Boundary
K12 is proposed as an application-facing trust boundary even when no concrete transport is implemented.

K12 must preserve and validate continuity for:

- caller identity reference
- authentication context reference
- authorization decision reference
- enterprise scope
- workspace scope
- project scope
- correlation identity
- causation identity
- audit evidence continuity
- command/query separation
- request/response pairing
- error disclosure boundaries
- capability admission

K12 must not:

- authenticate users itself
- authorize operations itself
- weaken K10 authentication or authorization semantics
- create a second identity model
- create a second authorization model
- create a second scope model
- create a second audit model
- create a second correlation model
- create a second command model
- create a second query model

K12 relies on frozen K10 evidence and K11 semantics.

## Frozen Semantics Preserved
K12 must preserve:

- K10 authentication semantics
- K10 authorization semantics
- K10 request validation semantics
- K10 rate-governance semantics
- K10 error semantics
- K10 correlation semantics
- K11 view semantics
- K11 command/query intent separation
- K11 scope continuity
- K11 audit continuity
- K11 projection semantics
- K11 technology neutrality

## Deferred Decisions Requiring Later ADRs
This ADR explicitly defers:

- transport and protocol selection
- async runtime ownership
- HTTP/REST architecture
- WebSocket/SSE architecture
- IPC architecture
- frontend framework
- browser application
- desktop application
- session persistence
- database and cache
- authentication-provider integration
- deployment and hosting topology
- TLS and ingress
- background synchronization
- event subscription runtime
- observability infrastructure

These are architectural decisions, not implementation details.

## Alternatives Considered
### Alternative A
`K12 Dashboard Runtime`

Rejected in this proposal because it is too UI-specific and prematurely assumes runtime ownership.

### Alternative B
`K12 Transport Layer`

Rejected in this proposal because it prematurely selects transport as the milestone responsibility.

### Alternative C
`K12 Application Integration`

Preferred in this proposal because it preserves technology neutrality and creates an application-facing coordination boundary above frozen K11.

### Alternative D
`Skip K12 and let frontend call K11 directly`

Rejected in this proposal because it leaks application coordination into frontend code and increases the risk of governance boundary erosion.

### Alternative E
`Add application concerns directly to kernel-studio`

Rejected in this proposal because it would reopen the frozen K11 boundary.

## Consequences
Positive:

- K11 remains frozen
- frontend technology remains replaceable
- transport remains replaceable
- application coordination receives a governed boundary
- future Web and Desktop applications can share semantics
- no infrastructure is introduced prematurely
- security evidence remains continuous

Negative:

- no live dashboard results from this phase alone
- no transport connectivity is introduced
- additional adapter/runtime ADRs will still be required
- some contracts may remain abstract until a concrete host is approved

## Implementation Authorization Effect
If accepted by human architecture authority, this ADR would authorize only:

- additive K12 application-coordination contracts
- additive K12 validation rules
- additive K12 continuity and envelope contracts
- additive K12 documentation and tests aligned to those contracts

Even after acceptance, this ADR does not authorize:

- transport runtime implementation
- frontend implementation
- persistence implementation
- session storage
- authentication-provider integration
- deployment or hosting implementation

## Approval Requirement
Human architecture approval remains required.

Until accepted:

- `K12 ADR: PROPOSED`
- `K12 ARCHITECTURE REVIEW: PENDING HUMAN APPROVAL`
- `K12 IMPLEMENTATION AUTHORIZATION: BLOCKED`
- `K12 IMPLEMENTATION: NOT STARTED`

## References
- [ARCHITECTURE.md](./../ARCHITECTURE.md)
- [DECISIONS.md](./../DECISIONS.md)
- [README.md](./../README.md)
- [docs/plans/K12-IMPLEMENTATION-PLAN.md](./plans/K12-IMPLEMENTATION-PLAN.md)
- [docs/backlog/K12-BACKLOG.md](./backlog/K12-BACKLOG.md)
- [docs/kernel-architecture/11-api-gateway-architecture.md](./kernel-architecture/11-api-gateway-architecture.md)
- [docs/kernel-architecture/12-studio-integration-architecture.md](./kernel-architecture/12-studio-integration-architecture.md)
- [docs/kernel-architecture/13-data-flow.md](./kernel-architecture/13-data-flow.md)
- [docs/kernel-architecture/15-roadmap.md](./kernel-architecture/15-roadmap.md)
- [docs/kernel-architecture/16-traceability.md](./kernel-architecture/16-traceability.md)
