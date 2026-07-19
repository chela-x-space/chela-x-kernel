# ADR-0003: K14 External Adapter Boundary

## Status
PROPOSED

## Date
2026-07-19

## Deciders
- Chief Enterprise Architect
- Kernel Platform Team

## Contributing Role
- Implementation Engineer

## Purpose
Define the smallest architecture-preserving external-adapter boundary
above frozen `K13 Service Integration` without introducing transport,
runtime, persistence, hosting, deployment, or infrastructure.

This ADR defines the K14 boundary only. It does not authorize transport
implementation, framework selection, runtime ownership, or crate
creation until human acceptance is recorded.

## Context
`K13 Service Integration` is closed and frozen under `ADR-0002`.

K13 establishes the authoritative service-facing coordination boundary
above `kernel-application`, but the repository still needs a governed
boundary for external callers or external adapter implementations.

That boundary must prevent transport, serialization, framework, and
other external concerns from leaking into `kernel-service`,
`kernel-application`, `kernel-studio`, `kernel-gateway`, or
`kernel-domain`.

K14 therefore defines contract-boundary semantics only. It is not a
transport implementation milestone.

## Decision
The proposed K14 milestone title is:

`K14 External Adapter Boundary`

The proposed K14 architectural role is:

`Technology-neutral external adapter boundary above frozen K13 Service Integration`

The proposed K14 layer position is:

```text
External System / External Adapter
                ↓
kernel-adapter
                ↓
kernel-service
                ↓
kernel-application
                ↓
kernel-studio
                ↓
kernel-gateway
                ↓
kernel-domain
```

K14 is additive only.

K14 remains contract-oriented only.

K14 is not:

- a transport implementation
- a runtime host
- a persistence subsystem
- a scheduler
- a deployment topology
- an infrastructure milestone

## Dependency Direction
The only proposed production dependency is:

```text
kernel-adapter -> kernel-service
```

K14 must consume frozen K13 meaning through `kernel-service`.

## Allowed Dependencies
Primary allowed production dependency:

- `kernel-service`

No direct production dependency is proposed on:

- `kernel-application`
- `kernel-studio`
- `kernel-gateway`
- `kernel-domain`

## Forbidden Dependencies
Forbidden production dependencies:

```text
kernel-adapter -> kernel-application
kernel-adapter -> kernel-studio
kernel-adapter -> kernel-gateway
kernel-adapter -> kernel-domain
```

Forbidden reverse dependencies:

```text
kernel-domain -> kernel-adapter
kernel-gateway -> kernel-adapter
kernel-studio -> kernel-adapter
kernel-application -> kernel-adapter
kernel-service -> kernel-adapter
```

## Adapter Responsibility
K14 adapter contracts may:

- translate external representations into frozen `kernel-service` contracts
- translate service outcomes into external-facing adapter outcomes
- preserve identity, scope, capability, correlation, audit, and compatibility continuity
- reject invalid external inputs before creating valid service requests

K14 adapter contracts do not own:

- business logic
- service orchestration
- domain mutation
- authorization policy decision
- persistence
- scheduling
- runtime execution
- transport hosting
- deployment

## Mandatory Ownership Rules
The following rules are normative:

- `Adapter MUST NOT own domain state.`
- `Adapter MUST NOT own service state.`
- `Adapter MUST NOT mutate kernel-domain directly.`
- `Adapter MUST NOT invoke kernel-application directly.`
- `Adapter MUST NOT invoke kernel-studio directly.`
- `Adapter MUST NOT invoke kernel-gateway directly.`
- `Adapter MUST NOT bypass kernel-service.`
- `Adapter MUST NOT redefine or weaken K13 validation semantics.`
- `Adapter MUST NOT fabricate successful service outcomes.`
- `Adapter MUST preserve request and response correlation.`
- `Adapter MUST preserve audit continuity.`
- `Adapter MUST remain replaceable.`

## Transport Neutrality
K14 must remain transport-neutral.

K14 public contracts must not encode transport-specific semantics tied
to:

- HTTP
- REST
- gRPC
- WebSocket
- GraphQL
- CLI
- Kafka
- RabbitMQ
- NATS
- JSON
- YAML
- XML
- Protobuf
- OpenAPI
- framework-specific request or response types

Primitive strings or immutable value references are allowed when they do
not introduce transport semantics.

## Planned Public Contract Categories
This ADR approves contract categories only and does not freeze final
type names:

- adapter API version
- adapter identity
- adapter kind
- adapter capability declaration
- adapter command intent
- adapter query intent
- adapter request context
- adapter request envelope
- adapter response envelope
- adapter compatibility reference
- adapter status snapshot
- adapter validation error

K14 must not define runtime implementation types.

## Validation Precedence
K14 validation must be deterministic.

Canonical invariants must be evaluated before downstream invariants.

The required precedence is:

1. adapter version
2. adapter identity
3. adapter capability
4. service compatibility
5. request identity and correlation continuity
6. command/query kind continuity
7. scope and audit continuity
8. response continuity

Implementation details may refine this order only if they do not violate
these precedence principles.

## Compatibility Requirements
K14 must preserve:

- K1-K13 frozen public APIs unchanged
- K13 as the authoritative service boundary
- no direct lower-layer bypass
- no reverse dependency from frozen lower crates

K14 is additive only.

Any incompatible change to K1-K13 requires a separate approved ADR.

## Forbidden Scope
K14 must not introduce:

- runtime
- Tokio
- async executor
- HTTP server
- REST router
- gRPC server
- WebSocket server
- message broker
- database
- persistence
- filesystem behavior
- scheduler
- queue
- cache
- plugin loader
- dynamic loading
- external API client
- AI model execution
- authentication implementation
- authorization engine
- deployment
- hosting
- infrastructure

## Consequences
Positive consequences:

- external concerns remain separated from `kernel-service`
- transport implementations remain replaceable
- K13 remains the authoritative service boundary
- validation continuity can be tested deterministically
- framework coupling is reduced

Trade-offs:

- transport-specific implementation must be deferred to a later milestone
- serialization mapping is not included in K14
- runtime wiring is not included in K14
- the system gains one additional contract layer

## Alternatives Rejected
1. Let external transport call `kernel-service` directly.
   Rejected because transport and external-representation concerns would
   mix into the frozen service boundary.

2. Let adapter call `kernel-application` or lower layers directly.
   Rejected because this would bypass K13.

3. Include HTTP or REST implementation inside K14.
   Rejected because it would break transport neutrality and introduce
   runtime or infrastructure before authorization.

4. Use `kernel-gateway` as the external adapter directly.
   Rejected because K10 is a frozen internal gateway contract below the
   studio, application, and service layers.

## ADR Status And Authorization
```text
ADR STATUS:
PROPOSED

K14 IMPLEMENTATION AUTHORIZATION:
BLOCKED PENDING HUMAN ACCEPTANCE
```
