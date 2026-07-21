# ADR-0004: K15 External Intake Trust Boundary

<!-- markdownlint-disable MD013 -->

- Status: PROPOSED
- Date: 2026-07-20
- Authors: CHELA-X Architecture
- Supersedes: None
- Superseded by: None

---

## Context

K14 establishes the External Adapter Boundary and freezes the public adapter contract exposed to external systems.

However, no architectural boundary currently exists to distinguish untrusted external intake from trusted adapter interaction.

External requests may originate from arbitrary networks, transports, identities, and integration mechanisms. Before any request reaches the K14 Adapter Boundary, the architecture requires a deterministic intake boundary responsible only for intake trust evaluation.

This milestone introduces the architectural definition of that boundary.

This ADR defines architecture only.

No implementation is authorized.

---

## Problem

External requests arrive from environments that cannot be assumed to be trusted.

Examples include:

- HTTP
- HTTPS
- gRPC
- WebSocket
- Message Queue
- CLI
- SDK
- Third-party Integration
- Future Protocols

These transports are intentionally outside Kernel trust.

The Kernel must never assume that an external request is valid merely because it reached an Adapter.

Therefore an architectural trust boundary is required before the K14 External Adapter Boundary.

---

## Decision

The CHELA-X Kernel SHALL define a new architectural boundary named:

### External Intake Trust Boundary

This boundary SHALL exist immediately before the External Adapter Boundary defined by K14.

All externally originated requests SHALL cross this boundary before they become eligible for adapter processing.

This boundary is responsible only for architectural trust evaluation.

It SHALL NOT perform business logic.

It SHALL NOT execute application behavior.

It SHALL NOT dispatch commands.

It SHALL NOT execute workflows.

It SHALL NOT mutate Kernel state.

---

## Architectural Responsibilities

The External Intake Trust Boundary is responsible for determining whether an incoming request is eligible to proceed toward the trusted architecture.

The boundary MAY evaluate architectural concerns including, but not limited to:

- Origin
- Transport
- Identity
- Authentication status
- Authorization context
- Request integrity
- Protocol validity
- Message structure
- Version compatibility
- Trust evidence

The architectural decision produced by this boundary SHALL be limited to one of the following outcomes:

- Accept
- Reject

No additional architectural behavior is defined by this ADR.

---

## Architectural Constraints

The External Intake Trust Boundary SHALL remain transport independent.

The External Intake Trust Boundary SHALL remain protocol independent.

The External Intake Trust Boundary SHALL remain identity-provider independent.

The External Intake Trust Boundary SHALL remain implementation independent.

This ADR intentionally defines only architectural responsibilities.

No implementation strategy is approved.

---

## Relationship to K14

The K14 External Adapter Boundary remains unchanged.

The External Intake Trust Boundary is positioned immediately before K14.

The processing sequence is therefore defined as:

Untrusted External Environment
→ External Intake Trust Boundary (K15)
→ External Adapter Boundary (K14)
→ Kernel Service Layer
→ Kernel Domain

K15 does not replace K14.

K15 establishes the architectural trust boundary.

K14 establishes the architectural adapter boundary.

These responsibilities are intentionally separated.

---

## Consequences

Positive consequences include:

- Explicit architectural separation between trusted and untrusted environments.
- Deterministic trust evaluation before adapter interaction.
- Reduced architectural coupling between transport technologies and Kernel services.
- Stable foundation for future authentication and authorization architecture.
- Clear trust model for all future external integrations.

Negative consequences include:

- An additional architectural boundary must be maintained.
- Future implementation work will require explicit trust evaluation before adapter processing.

---

## Alternatives Considered

## Alternative 1 — Trust Evaluation Inside Adapters

Rejected.

Embedding trust evaluation inside individual adapters would duplicate responsibilities across transports and violate architectural separation of concerns.

---

## Alternative 2 — Trust Evaluation Inside Kernel Services

Rejected.

Kernel Services are part of the trusted architecture and therefore SHALL NOT be responsible for determining whether an external request is trustworthy.

---

## Alternative 3 — No Explicit Trust Boundary

Rejected.

Without an explicit architectural trust boundary, the transition between untrusted environments and trusted Kernel components becomes ambiguous and difficult to govern consistently.

---

## Compatibility

This ADR introduces a new architectural boundary only.

No existing public API is modified.

No K14 interface is changed.

No Adapter contract is changed.

No Service contract is changed.

No Domain contract is changed.

Backward compatibility with K14 is fully preserved.

---
---

## Decision Drivers

The architectural decision defined by this ADR is driven by the following
enterprise architecture objectives:

- Establish a deterministic trust boundary between external environments and
  trusted Kernel components.
- Separate trust evaluation from adapter execution.
- Preserve the K14 External Adapter Boundary without modification.
- Minimize coupling between transport technologies and Kernel architecture.
- Provide a stable architectural foundation for future authentication,
  authorization, gateway, and identity capabilities.
- Ensure all future external integrations follow a consistent trust model.

This ADR intentionally defines architectural direction only.

---

## Architecture Invariants

The following architectural properties SHALL remain invariant unless superseded
by a future Accepted ADR.

The External Intake Trust Boundary:

- SHALL remain stateless.
- SHALL execute before every External Adapter interaction.
- SHALL NOT become part of the Kernel Domain.
- SHALL NOT perform business processing.
- SHALL NOT execute workflows.
- SHALL NOT dispatch commands.
- SHALL NOT mutate Kernel state.
- SHALL NOT establish business ownership.
- SHALL NOT bypass the External Adapter Boundary.
- SHALL remain independent from transport technologies.
- SHALL remain independent from implementation technologies.
- SHALL remain independent from identity-provider implementations.

Violation of these invariants constitutes an architectural violation.

---

## Compliance Rules

Future architectural components SHALL comply with the following rules.

External integrations:

- SHALL terminate at the External Intake Trust Boundary.
- SHALL NOT communicate directly with Kernel Services.
- SHALL NOT communicate directly with the Kernel Domain.
- SHALL pass through the K15 Trust Boundary before reaching K14.

Future Adapter implementations:

- SHALL assume trust evaluation has already completed.
- SHALL NOT repeat architectural trust-boundary responsibilities.
- SHALL remain focused on adapter responsibilities defined by ADR-0003.

Future architectural changes SHALL preserve this separation of concerns unless
explicitly superseded by a future Accepted ADR.

---

## Future Architectural Impact

This ADR establishes the architectural foundation for future Kernel milestones.

Future ADRs MAY extend this boundary to define:

- Authentication architecture.
- Authorization architecture.
- Identity architecture.
- Trust evidence architecture.
- API Gateway architecture.
- External Gateway policy.
- Federation architecture.
- Zero Trust architectural model.

Such ADRs SHALL extend this architectural boundary without redefining or
removing the responsibilities established by this ADR.

---

## Scope

This ADR defines architecture only.

This ADR does not define:

- Authentication implementation
- Authorization implementation
- Identity providers
- Access control mechanisms
- Cryptographic algorithms
- Network security configuration
- API Gateway behavior
- Adapter implementation
- Service implementation
- Runtime enforcement
- Operational policies

These concerns MAY be defined by future ADRs.

---

## Status

PROPOSED

This ADR becomes authoritative only after Human Architecture Approval.

Until approved, K15 remains in planning status.

---

## References

This ADR builds upon the following architectural decisions:

- ADR-0001 — Kernel Architecture Baseline
- ADR-0002 — Kernel Service Boundary
- ADR-0003 — K14 External Adapter Boundary

This ADR does not supersede any previous architectural decision.

Future ADRs defining authentication, authorization, identity, gateway behavior, or transport-specific trust mechanisms SHALL conform to the architectural boundary established by this ADR.

---

## Traceability

| Requirement | Description | Status |
| --- | --- | --- |
| K15-001 | Define an explicit External Intake Trust Boundary | Defined |
| K15-002 | Separate untrusted intake from trusted adapter processing | Defined |
| K15-003 | Preserve K14 External Adapter Boundary | Preserved |
| K15-004 | Prohibit business execution within the trust boundary | Defined |
| K15-005 | Maintain transport-independent architecture | Defined |
| K15-006 | Maintain implementation independence | Defined |

---

## Decision Record

Decision: Introduce an architectural trust boundary before the K14 External Adapter Boundary.

Decision Type: Architecture

Impact: Additive

Breaking Change: No

Implementation Authorized: No

Architecture Status: Proposed

---

## Approval

| Role | Status |
| --- | --- |
| Chief Enterprise Architect | Pending |
| Human Architecture Review | Pending |
| Kernel Architecture Board | Pending |

---

## Notes

This Architectural Decision Record defines the existence, purpose, and architectural responsibilities of the External Intake Trust Boundary.

No runtime behavior, implementation detail, technology selection, protocol specification, authentication mechanism, authorization mechanism, or security product is approved by this document.

Any future implementation SHALL require explicit approval through subsequent Architectural Decision Records.

---
