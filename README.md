# CHELA-X Kernel

## Status
Implementation (K8 Closed, K9 Closed, K10 Closed, K11 Closed, K12 ADR Proposed)

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-19

## Applies To
CHELA-X Kernel repository baseline, bootstrap, and future implementation work.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose
CHELA-X Kernel is the governed implementation repository for the kernel layer that sits between CHELA-X CES and future CHELA-X Runtime.

## Dependency Position
AI Engineering OS -> CHELA-X CES -> CHELA-X Kernel -> CHELA-X Runtime -> CHELA-X SDK -> CHELA-X Media

## Baseline
- AI Engineering OS v1.0 Freeze defines global engineering, workflow, decision, architecture, and metadata rules.
- CHELA-X CES Book 0 RC1 defines the canonical enterprise requirements consumed by Kernel.
- CHELA-X Program defines product dependency direction and portfolio ordering.
- CHELA-X Library provides the read-only retrieval index used for traceability and verification.

## Current Kernel Status

| Milestone | Status |
|-----------|--------|
| K1 Domain Foundation | PASS |
| K2 State Lifecycle | PASS |
| K3 Authorization Foundation | PASS |
| K4 Runtime Supervision | PASS |
| K5.1 Canonical Event Envelope | PASS |
| K5.2 Validation | PASS |
| K5.3 Event Streams | PASS |
| K5.4 Replay | PASS |
| K5 Enterprise Event System | PASS / COMPLETE |
| K6 Workflow Engine | PASS |
| K7 Task Engine | PASS / COMPLETE |
| K8 Execution Engine | PASS / COMPLETE |
| K9 Enterprise Memory | PASS / COMPLETE |
| K10 API Gateway | PASS / COMPLETE |
| K11 Studio Integration | PASS / COMPLETE |

Canonical host validation:

- **877 passed**
- **0 failed**

## Current Status
`K8 Execution Engine Closed And Frozen; K9 Enterprise Memory Closed And Frozen; K10 API Gateway Closed And Frozen For K11 Consumption; K11 Studio Integration Closed And Frozen For K12 Consumption; K12 Planning Complete And ADR Proposed Pending Human Approval`

## Constraints
- Architecture is frozen.
- No redesign may occur without an approved ADR.
- K1 through K7 are complete and remain compatible with K8.
- K6 workflow implementation is additive in `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/state.rs`, and existing `kernel-domain` re-exports.
- K7 task implementation remains frozen for next-milestone consumption.
- K8 execution implementation is additive in `crates/kernel-domain/src/execution*.rs`, `crates/kernel-domain/src/errors.rs`, and existing `kernel-domain` re-exports.
- Canonical host validation passed with `877 passed`, `0 failed`, `0 ignored`.
- K6 Workflow Engine domain layer is complete.
- K6 is deterministic and side-effect free.
- K6 public API is frozen for downstream consumption.
- K7 public API is frozen for next-milestone consumption.
- K8 execution-domain API is frozen for next-milestone consumption.
- K9 implementation is complete, native verification passed, and K9 public API is frozen for K10 consumption.
- K10 planning is complete and architecture review passed.
- K10 implementation is complete, architecture review passed, compile validation passed, and native verification passed on the primary host.
- K10 introduces additive transport-neutral API Gateway contracts in `crates/kernel-gateway` without transport infrastructure.
- K10 public API is frozen for K11 consumption.
- K11 planning is complete and architecture review passed.
- K11 implementation is complete.
- K11 compile validation passed in the repository workspace.
- K11 native verification passed on the primary host.
- K11 public API is frozen for K12 consumption.
- K12 planning is complete.
- K12 ADR is proposed.
- K12 architecture review is pending human approval.
- ADR acceptance is required before K12 implementation authority exists.
- K12 implementation is not started.
- K6 preserves the architecture freeze.
- K8 preserves the architecture freeze.
- Runtime execution is not implemented.
- Domain API is frozen for downstream consumption.
- No business logic, persistence, networking, or workflow execution is introduced in K1.
- No workflow execution, persistence, networking, runtime orchestration, scheduler, event bus, or worker infrastructure is introduced in K6.
- No frozen upstream repository may be modified by this repository.

## Domain Scope
- Stable identifiers
- Enterprise ownership paths
- Immutable identity primitives
- Lifecycle state types
- State snapshots, transition requests, transition outcomes, lifecycle guards, and failure codes
- Authorization request record types
- Decision record types
- Authorization, agent, delegation, policy, and workflow reference types
- Deterministic authorization enforcement inputs, traces, results, and decision construction helpers
- Deterministic runtime registry, capability indexing, heartbeat, freshness, lease, presence, runtime-health, runtime-snapshot, and supervisor primitives
- Deterministic workflow foundation, definition, instance, transition-control, step-coordination, authorization-integration, event-integration, and failure-or-recovery primitives
- Deterministic task foundation, definition, instance, ownership, assignment, priority, readiness, lifecycle, dependency, completion, failure, evidence, and integration primitives
- Deterministic execution request, context, session, outcome, evidence-binding, retry-eligibility, and audit-reference primitives
- Deterministic enterprise-memory identity, provenance, classification, retention, retrieval, and read-only projection primitives
- Deterministic API Gateway contract identity, authentication context, authorization binding, request validation, response mapping, error translation, status snapshot, and protocol-adaptation primitives
- Deterministic Studio Integration contract identity, view coordination, projection intent, command-console mapping, validation, and K10 boundary-conformance primitives

## References
- [AGENTS.md](./AGENTS.md)
- [ENGINEERING.md](./ENGINEERING.md)
- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [docs/BASELINE.md](./docs/BASELINE.md)
- [docs/TRACEABILITY.md](./docs/TRACEABILITY.md)
- [docs/IMPLEMENTATION-PLAN.md](./docs/IMPLEMENTATION-PLAN.md)
- [docs/K2-STATE-LIFECYCLE.md](./docs/K2-STATE-LIFECYCLE.md)
- [docs/K3-DECISION-AUTHORIZATION.md](./docs/K3-DECISION-AUTHORIZATION.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
