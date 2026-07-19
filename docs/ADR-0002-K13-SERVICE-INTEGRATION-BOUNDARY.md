# ADR-0002: K13 Service Integration Boundary

## Status
ACCEPTED

## Date
2026-07-19

## Deciders
- Chief Enterprise Architect
- Kernel Platform Team

## Contributing Role
- Implementation Engineer

## Purpose
Define the smallest architecture-preserving service-facing boundary above
frozen `K12 Application Integration` without introducing runtime,
transport, persistence, scheduler, or other infrastructure.

This ADR defines what K13 is before any K13 implementation, crate
creation, or dependency expansion is attempted.

## Context
The frozen CHELA-X Kernel architecture baseline currently defines
approved milestones through `K11 Studio Integration`.

`K12 Application Integration` exists only because `ADR-0001` approved a
new boundary above K11.

The repository now needs an architectural decision that determines
whether a service-facing coordination layer may exist above K12 while
preserving:

- frozen K1-K12 public APIs
- existing dependency direction
- deterministic contract-only behavior
- zero runtime or infrastructure ownership

## Layering
The accepted K13 layer position is:

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

K13 is above `kernel-application` and below any concrete service adapter
or host.

## Decision
The accepted official K13 title is:

`K13 Service Integration`

The accepted architectural role is:

`Technology-neutral service coordination boundary above K12 Application Integration`

K13 is a contract-oriented service boundary only.

K13 is not:

- a transport layer
- a runtime host
- a persistence subsystem
- a scheduler
- a worker runtime
- a concrete service process
- a deployment topology

## Dependency Direction
The accepted dependency direction is:

```text
kernel-service
    -> kernel-application
    -> kernel-studio
    -> kernel-gateway
    -> kernel-domain
```

K13 must consume lower-layer meaning through `kernel-application`.

K13 must not bypass K12, K11, or K10.

## Allowed Dependencies
Primary allowed dependency:

- `kernel-application`

Exceptions still requiring explicit future approval if ever requested:

- none approved in this ADR

This ADR intentionally does not authorize direct K13 dependencies on
`kernel-studio`, `kernel-gateway`, or `kernel-domain`.

## Forbidden Dependencies
Forbidden reverse dependencies:

```text
kernel-domain -> kernel-service
kernel-gateway -> kernel-service
kernel-studio -> kernel-service
kernel-application -> kernel-service
```

Forbidden bypasses:

```text
kernel-service -> kernel-domain direct mutation
kernel-service -> K10 bypass
kernel-service -> K11 bypass
kernel-service -> K12 bypass
external service adapter -> direct kernel-domain access
external service adapter -> bypass K10
external service adapter -> bypass K11
external service adapter -> bypass K12
```

## Service Ownership Rules
K13 service integration owns:

- service-facing coordination contracts
- service identity references
- service request and response envelopes
- service capability declarations
- service continuity validation over K12 contracts

K13 service integration does not own:

- runtime lifecycle
- network listeners
- transport protocols
- persistence
- scheduling
- background execution
- authentication providers
- authorization engines
- deployment concerns

## Service State Rules
Mandatory decision:

`Service owns no persistent state.`

K13 may define explicit service session or status references only as
caller-supplied, immutable contract values.

K13 must not:

- store service sessions
- persist service state
- cache service state
- infer state from environment or clock
- create hidden mutable state

## Domain Mutation Rules
Mandatory decisions:

- `Service cannot bypass kernel-application.`
- `Service cannot mutate kernel-domain directly.`

All K13 command and query coordination must flow through frozen K12
contracts, which in turn preserve the frozen K11 and K10 boundaries.

K13 must not:

- call `kernel-domain` to mutate state
- create parallel command semantics
- create parallel query semantics
- reinterpret lower-layer authorization or audit meaning

## Service Replaceability Rules
Mandatory decisions:

- `Service contracts remain technology-neutral.`
- `Service layer is replaceable without changing K12 APIs.`

K13 must remain replaceable by:

- different service hosts
- different transport adapters
- different deployment topologies

without forcing any change to frozen `kernel-application` APIs.

K13 replaceability requires:

- no transport-specific metadata in service contracts
- no runtime-specific ownership semantics
- no persistence-specific semantics
- no infrastructure-specific dependency assumptions

## Compatibility Requirements
K13 must preserve:

- K1-K12 public APIs unchanged
- K12 as the only approved application coordination boundary
- K11 Studio contract semantics
- K10 Gateway authentication, authorization, validation, and error semantics
- K1-K12 identity, scope, correlation, and audit continuity

K13 must not create:

- a second application model
- a second service authorization model
- a second scope model
- a second correlation model
- a second audit model

## Consequences
Positive consequences:

- service-facing coordination gains a governed architectural place
- K12 remains the only application-facing coordination boundary
- transport and hosting remain replaceable
- K13 can remain contract-only and infrastructure-free
- frozen K1-K12 APIs remain unchanged

Negative consequences:

- K13 implementation may proceed within this accepted ADR boundary
- no concrete service host or transport is authorized by this ADR
- later ADRs will still be required for transport, runtime, persistence,
  deployment, or observability concerns

## Repository Architecture Preservation
This ADR did not:

- implement K13
- create `crates/kernel-service`
- modify `Cargo.toml`
- modify frozen APIs
- change dependency direction in code

Repository architecture remained identical at acceptance time; later K13
implementation still had to preserve the accepted dependency direction,
frozen lower-layer APIs, and infrastructure-free contract scope.
