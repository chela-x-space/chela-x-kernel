# ADR-0004: K15 External Intake Trust Boundary

- Status: PROPOSED
- Date: 2026-07-20
- Authors: CHELA-X Architecture
- Supersedes: None
- Superseded by: None

---

# Context

K14 establishes the External Adapter Boundary and freezes the public adapter contract exposed to external systems.

However, no architectural boundary currently exists to distinguish untrusted external intake from trusted adapter interaction.

External requests may originate from arbitrary networks, transports, identities, and integration mechanisms. Before any request reaches the K14 Adapter Boundary, the architecture requires a deterministic intake boundary responsible only for intake trust evaluation.

This milestone introduces the architectural definition of that boundary.

This ADR defines architecture only.

No implementation is authorized.

---

# Problem Statement

Without an intake trust boundary:

- External requests reach adapter contracts without architectural intake validation.
- Claimed identities can be confused with verified identities.
- Trust classification becomes inconsistent.
- Audit continuity cannot be guaranteed.
- External transport details risk leaking into downstream layers.

The architecture requires a dedicated trust boundary before K14.

---

# Decision

Introduce a new architectural boundary named:

**External Intake Trust Boundary (K15)**

This boundary SHALL exist conceptually before K14.

```
External System
        │
        ▼
External Intake Trust Boundary (K15)
        │
        ▼
External Adapter Boundary (K14)
        │
        ▼
Kernel Services
```

This ADR defines architectural contracts only.

Implementation remains prohibited.

---

# Boundary Responsibilities

K15 SHALL be responsible for:

- Intake validation
- Claimed identity capture
- Observed source capture
- Trust classification
- Correlation continuity
- Audit continuity
- Request acceptance or rejection

K15 SHALL NOT perform:

- Authentication
- Authorization
- Business validation
- Service execution
- Adapter execution
- Domain logic

---

# Mandatory Rules

The following rules are mandatory.

## Identity

ClaimedIdentity SHALL NOT become VerifiedIdentityReference.

Verification SHALL occur outside K15.

---

## Trust

Trust classification SHALL NOT imply authorization.

Accepted intake SHALL NOT imply authenticated identity.

---

## Audit

All intake decisions SHALL preserve audit continuity.

Correlation identifiers SHALL remain stable across downstream boundaries.

---

## Security

Raw credentials SHALL NOT appear in public contracts.

Secrets SHALL NOT cross architectural boundaries.

---

## Transport

K15 SHALL remain transport neutral.

The architecture SHALL NOT depend on:

- HTTP
- HTTPS
- gRPC
- WebSocket
- Message Queue
- CLI
- SDK

---

## Dependency

K15 SHALL forward accepted requests only to K14.

K15 SHALL NOT bypass K14.

---

# Non-Goals

This ADR does not introduce:

- Authentication providers
- Authorization engines
- OAuth
- JWT
- OIDC
- API Gateway implementation
- Network stack
- Runtime services
- Persistence
- Message broker

---

# Compatibility

This ADR is additive.

No changes are made to:

- K1
- K2
- K3
- K4
- K5
- K6
- K7
- K8
- K9
- K10
- K11
- K12
- K13
- K14

Existing public APIs remain unchanged.

---

# Verification Requirements

Human Architecture Review SHALL confirm:

- Architecture consistency
- Dependency direction
- Trust separation
- API compatibility
- Transport neutrality
- Audit continuity
- Architecture Freeze compliance

Implementation SHALL NOT begin until:

- ADR status becomes ACCEPTED.
- Human implementation authorization is granted.

---

# Approval

Status:

PROPOSED

Implementation Authorization:

NOT GRANTED

Architecture Freeze:

ACTIVE