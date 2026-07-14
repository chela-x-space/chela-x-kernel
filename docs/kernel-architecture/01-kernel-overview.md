# 01 — CHELA-X Kernel Overview

## Metadata

- Status: ACTIVE ARCHITECTURE BASELINE
- Book: CHELA-X Kernel Architecture
- Chapter: 01
- Architecture State: FROZEN
- Applies To: K1–K11
- Repository: `chela-x-kernel`
- Architecture Authority: Chief Enterprise Architect
- Implementation Authority: Implementation Engineer
- ADR Required: Only for architecture changes

---

## 1. Purpose

The CHELA-X Kernel is the governed execution foundation of the CHELA-X Operating System.

It provides deterministic domain rules, lifecycle control, authorization enforcement, runtime governance, enterprise events, workflow coordination, task governance, execution control, memory integration, API exposure, and Studio connectivity.

The Kernel does not represent a user interface, business application, content generator, or autonomous agent by itself.

It provides the trusted rules and canonical contracts through which those systems operate.

---

## 2. Architectural Position

```text
CHELA-X Enterprise Specification
                │
                ▼
        CHELA-X Kernel
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
     Runtime  Workflow  Decision
        │       │        │
        └───────┼────────┘
                ▼
           Task Engine
                │
                ▼
        Execution Engine
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
      Events  Memory    Audit
                │
                ▼
          API Gateway
                │
                ▼
        CHELA-X Studio
```

CES defines enterprise meaning, authority, policy, governance, and organizational semantics.

The Kernel converts those approved rules into deterministic and testable domain behavior.

---

## 3. Kernel Mission

The Kernel exists to ensure that enterprise actions remain:

- Identifiable
- Authorized
- Deterministic
- Validated
- Traceable
- Auditable
- Replayable where permitted
- Governed throughout their lifecycle

The Kernel must prevent agents, workflows, APIs, Studio commands, or external systems from bypassing approved enterprise rules.

---

## 4. Kernel Capability Map

| Milestone | Capability | Primary Responsibility | Status |
|---|---|---|---|
| K1 | Domain | Canonical enterprise identities, value objects, and domain invariants | PASS |
| K2 | Lifecycle | Validated state transitions and terminal-state protection | PASS |
| K3 | Authorization / Enforcement | Deterministic authority evaluation and enforcement | PASS |
| K4.1 | Runtime Registry | Canonical runtime registration and runtime identity | PASS |
| K4.2 | Runtime Supervision | Heartbeat, lease, health, recovery, and supervision outcomes | PASS |
| K5 | Enterprise Event System | Canonical enterprise event facts and event history | PLANNED |
| K6 | Workflow Engine | Governed orchestration of enterprise processes | PLANNED |
| K7 | Task Engine | Governed units of work, ownership, assignment, and task state | PLANNED |
| K8 | Execution Engine | Controlled execution of approved work | PLANNED |
| K9 | Memory | Governed operational and organizational memory | PLANNED |
| K10 | API Gateway | External contract exposure and boundary enforcement | PLANNED |
| K11 | Studio Integration | Realtime command-center integration | PLANNED |

---

## 5. Accepted Baseline

K1 through K4.2 are accepted implementation baselines.

They must not be redesigned, reinterpreted, or replaced during K5 development.

K5 must extend the existing Kernel through additive architecture and approved dependency direction.

Canonical host validation for the accepted baseline is:

```text
146 passed
0 failed
0 ignored
```

The following gates are accepted:

```text
cargo fmt
cargo check
cargo test
cargo clippy
cargo doc
cargo test --doc
```

---

## 6. Kernel Boundaries

### 6.1 The Kernel Owns

- Canonical domain invariants
- Lifecycle validation
- Authorization enforcement
- Runtime identity and supervision rules
- Canonical event validation
- Workflow and task state governance
- Execution eligibility and execution outcomes
- Memory contracts and provenance requirements
- API boundary contracts
- Studio command validation

### 6.2 The Kernel Does Not Own

- User-interface rendering
- Dashboard layout
- Model training
- LLM inference implementation
- Image or video generation engines
- Social-platform adapters
- Business-specific content templates
- Infrastructure provisioning
- Operating-system process scheduling
- Unvalidated external data

These capabilities may integrate with the Kernel but must not be embedded into Kernel domain rules.

---

## 7. Layered Architecture

```text
Layer 7  Studio and External Clients
Layer 6  API Gateway and Integration Contracts
Layer 5  Memory, Audit, and Projection Interfaces
Layer 4  Execution Engine
Layer 3  Workflow and Task Engines
Layer 2  Enterprise Event System
Layer 1  Runtime, Authorization, and Lifecycle
Layer 0  Canonical Domain
```

Each higher layer may depend on approved contracts from lower layers.

Lower layers must not depend on Studio, external clients, or presentation concerns.

Circular dependencies are prohibited.

---

## 8. Dependency Direction

The canonical dependency direction is:

```text
Studio
  ↓
API Gateway
  ↓
Workflow / Task / Execution
  ↓
Enterprise Events
  ↓
Runtime / Authorization / Lifecycle
  ↓
Domain
```

Event records may describe outcomes from any approved capability, but the Event System must not bypass the domain, lifecycle, or authorization layers.

---

## 9. Deterministic Core

Kernel domain operations must be deterministic for equivalent validated inputs.

The deterministic core must not directly depend on:

- Wall-clock acquisition
- Random number generation
- Network access
- Database access
- Filesystem access
- Environment variables
- External model inference

Required values such as timestamps, identifiers, policies, observations, and evidence must be supplied through explicit validated inputs.

Infrastructure layers may acquire those values but must not conceal them from domain validation.

---

## 10. State and Facts

The Kernel distinguishes between state and facts.

- State represents the current validated condition of an entity.
- Events represent immutable facts that describe accepted occurrences.
- Commands represent requested actions and are not facts.
- Decisions represent governed determinations.
- Evidence supports validation, authorization, audit, and traceability.

A request must not be recorded as a successful event unless its outcome has been validated and accepted.

---

## 11. Runtime Relationship

The Runtime layer governs registered digital workers, services, agents, and execution-capable components.

K4.1 establishes runtime identity and registration.

K4.2 establishes deterministic supervision, heartbeat validation, lease assessment, health classification, recovery eligibility, and supervisor outcomes.

Runtime supervision does not execute agents and does not publish events directly.

K5 will provide canonical event representation for accepted runtime occurrences without changing K4 responsibilities.

---

## 12. Event Relationship

K5 introduces the Enterprise Event System.

Its first responsibility is the Canonical Event Envelope.

Every accepted enterprise event must have a stable identity, type, source, subject, occurrence time, recording time, schema version, correlation context, causation context, security classification, evidence references, and immutable payload.

The Event System records accepted facts. It does not decide authorization, execute workflows, or mutate historical events.

---

## 13. Workflow and Task Relationship

K6 coordinates governed business processes.

K7 represents governed units of work.

Workflows may create, assign, transition, suspend, resume, complete, fail, or cancel tasks only through approved rules.

Workflow state and task state must remain distinguishable.

A workflow describes process coordination.

A task describes accountable work.

---

## 14. Execution Relationship

K8 performs approved execution.

Execution must be separated from planning, authorization, and decision authority.

The Execution Engine must consume validated work, produce explicit outcomes, and emit evidence suitable for events and audit.

Execution failure must not silently rewrite workflow, task, or decision history.

---

## 15. Memory Relationship

K9 provides governed memory interfaces.

Memory is not unrestricted storage.

Every memory item must preserve identity, provenance, classification, retention policy, access policy, and relationship to its originating facts or decisions.

Memory must not become an alternate source of authority that bypasses CES or Kernel rules.

---

## 16. API Relationship

K10 exposes approved Kernel capabilities through stable contracts.

The API Gateway must validate identity, authority, input shape, schema version, classification, and request context before invoking Kernel operations.

API convenience must not weaken Kernel invariants.

External errors must map from canonical Kernel outcomes without hiding their meaning.

---

## 17. Studio Relationship

K11 connects CHELA-X Studio to the Kernel.

Studio is an Enterprise Command Center rather than a CRUD dashboard.

Its primary operating model is:

```text
Top View
↓
Digital Twin
↓
Realtime Runtime
↓
Workflow
↓
Tasks
↓
Events
↓
Audit
↓
Decision
↓
Revenue
```

Studio may observe and request actions, but Kernel validation remains authoritative.

Visual state must be derived from canonical Kernel state, events, and approved projections.

---

## 18. Architecture Invariants

The following invariants apply across K1–K11:

1. Every governed entity has canonical identity.
2. Every state transition is explicitly validated.
3. Every protected action requires sufficient authority.
4. Terminal state cannot be silently reversed.
5. Runtime identity is separate from agent identity.
6. Commands are not events.
7. Events are immutable after acceptance.
8. Correlation does not imply causation.
9. Workflow ownership is separate from execution authority.
10. Task assignment is separate from task approval.
11. Execution outcomes preserve evidence.
12. Memory preserves provenance.
13. APIs cannot bypass domain rules.
14. Studio is not a source of canonical truth.
15. Architecture changes require an approved ADR.

---

## 19. Failure Model

Kernel operations must fail explicitly.

Failures must be represented through canonical error or outcome types rather than hidden side effects.

Validation precedence must be deterministic so that equivalent invalid inputs produce equivalent failure classifications.

A failure at a lower architectural layer must not be reclassified by a higher layer in a way that changes its canonical meaning.

---

## 20. Evolution Model

Kernel evolution is additive by default.

New milestones must:

1. Preserve accepted baselines.
2. Define explicit scope and exclusions.
3. Define domain types and invariants before infrastructure.
4. Maintain CES traceability.
5. Include deterministic tests.
6. Pass all engineering gates.
7. Avoid architecture changes unless approved by ADR.

---

## 21. Next Milestone

The next milestone is:

```text
K5 — Enterprise Event System
K5.1 — Canonical Event Envelope
```

K5 implementation must not begin until the Enterprise Event Model Specification has been written, reviewed, and approved.

---

## 22. Canonical Determination

CHELA-X Kernel is the deterministic governance and execution foundation of CHELA-X OS.

K1 through K4.2 are preserved as accepted baselines.

K5 through K11 must extend this baseline through approved contracts and dependency direction.

This chapter is the canonical overview for all subsequent Kernel architecture chapters.
