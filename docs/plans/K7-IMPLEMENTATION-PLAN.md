# K7 Implementation Plan

## K7 Milestone
COMPLETE

## Architecture State
FROZEN

## Specification Package
ARCHITECTURE APPROVED

## Architecture Review
PASS

## Implementation
COMPLETE

## API
FROZEN FOR NEXT-MILESTONE CONSUMPTION WITH NATIVE VERIFICATION BLOCKER

## Dependency Direction

```text
Workflow Engine
    ↓
Task Engine
    ↓
Future Execution Engine
```

## Architecture Review Gate

Before `K7-001` Rust implementation begins, the K7 specification package MUST receive an architecture review.

The architecture review MUST verify:

- K7 remains Task Engine
- task semantics align with `docs/kernel-architecture/08-task-architecture.md`
- dependency direction remains `Workflow → Task → Execution`
- K6 workflow API remains unchanged
- K7 does not execute tasks
- K7 does not introduce infrastructure
- K7 can reuse frozen K1-K6 domain primitives
- no ADR is required for the planned additive domain work
- missing CES IDs are tracked without fabrication

## Work-Item Sequence

### K7-001 Task Engine Foundation
- Objective: define stable task vocabulary, identity, references, and foundational validation expectations
- Authoritative specification: `docs/specifications/K7.1-task-engine-foundation.md`
- Allowed production files: `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs`, `crates/kernel-domain/src/workflow.rs`, new additive task-domain files under `crates/kernel-domain/src` only if architecture review approves them
- Likely public API surface: `TaskDefinitionId`, `TaskInstanceId`, `TaskDependencyId`, `TaskEvidenceId`, explicit reference types only where needed, foundational task result vocabulary, task-related `DomainError` variants if required
- Explicit non-goals: execution, scheduler, persistence, event bus, network, async runtime
- Upstream compatibility obligations: K1 identifiers, K2 transition patterns, K6 frozen API surface unchanged
- Test groups: `task_engine_foundation_*`, compatibility tests for K1-K6 consumed types
- Validation gates: fmt, check, test, clippy, doc, doc tests, clean diff
- Traceability requirement: architecture traceability plus `CES TRACEABILITY: PENDING AUTHORITATIVE MAPPING` until authoritative IDs are available
- Completion criteria: stable task vocabulary defined in code without public `TaskId` and without infrastructure semantics
- Commit boundary: `feat(task): add task engine foundation`
- Current status: `IMPLEMENTED — API REVIEW PASSED`

### K7-002 Task Definition
- Objective: add immutable task-definition model
- Authoritative specification: `docs/specifications/K7.2-task-definition.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: `TaskDefinition`, version, contracts, requirements, evidence requirements, failure-policy reference
- Explicit non-goals: live runtime state, execution, scheduler
- Upstream compatibility obligations: K6 workflow-definition compatibility, K1 version conventions
- Test groups: `task_definition_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: immutable definition semantics with deterministic rejection coverage
- Commit boundary: `feat(task): add task definition model`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-003 Task Instance
- Objective: add immutable task-instance model
- Authoritative specification: `docs/specifications/K7.3-task-instance.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: `TaskInstance`, definition snapshot reference, workflow binding, step binding, creation context
- Explicit non-goals: execution, persistence, implicit event publication
- Upstream compatibility obligations: K6 workflow-instance and workflow-step compatibility
- Test groups: `task_instance_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: deterministic instance creation and immutable snapshot binding
- Commit boundary: `feat(task): add task instance model`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-004 Task Ownership And Assignment
- Objective: define accountable ownership and future-execution assignment model
- Authoritative specification: `docs/specifications/K7.4-task-ownership-and-assignment.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: task ownership, assignment, assignment statuses, assignment reasons, deterministic decision outcomes, authority-scoped control
- Explicit non-goals: policy redefinition, execution, scheduler
- Upstream compatibility obligations: K3 authorization outcomes, K4 runtime and supervision facts, K6 workflow compatibility
- Test groups: `task_ownership_*`, `task_assignment_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: explicit owner and assignee semantics with eligibility validation, with `Assigned` used only as assignment status
- Commit boundary: `feat(task): add K7 task ownership and assignment`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-005 Task Priority And Readiness
- Objective: define deterministic priority and readiness evaluation
- Authoritative specification: `docs/specifications/K7.5-task-priority-and-readiness.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: `TaskPriority`, `TaskReadiness`, `TaskReadinessDecision`, `TaskReadinessBlocker`
- Explicit non-goals: scheduler, resource reservation, execution
- Upstream compatibility obligations: K3 authorization reuse, K4 runtime fact reuse, K6 workflow-state non-mutation
- Test groups: `task_priority_readiness_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: side-effect-free readiness evaluation with `Ready` and `Blocked` only as derived readiness outcomes
- Commit boundary: `feat(task): add priority and readiness validation`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-006 Task Lifecycle And State
- Objective: define task lifecycle vocabulary and transition control
- Authoritative specification: `docs/specifications/K7.6-task-lifecycle-and-state.md`
- Allowed production files: additive task-domain files, `state.rs` only if architecture review confirms reuse pattern, `lib.rs`, `errors.rs`
- Likely public API surface: task state, snapshots, transition requests, transition outcomes, transition validator using canonical states `Pending`, `InProgress`, `Completed`, `Failed`, `Cancelled`, `Archived`
- Explicit non-goals: workflow-state redesign, persistence, event emission
- Upstream compatibility obligations: K2 lifecycle conventions, K6 workflow semantics unchanged
- Test groups: `task_lifecycle_state_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: deterministic task transitions with the frozen transition map, no-op handling, sequence-mismatch rejection, and terminal-state protection
- Commit boundary: `feat(task): add lifecycle and transition control`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-007 Task Dependency Coordination
- Objective: define task-to-task dependency coordination
- Authoritative specification: `docs/specifications/K7.7-task-dependency-coordination.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: task dependency types, validation outcomes, graph references
- Explicit non-goals: scheduler, automatic downstream mutation, execution
- Upstream compatibility obligations: K6 workflow-step ordering preserved, K5 evidence patterns reused by reference only
- Test groups: `task_dependency_coordination_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: explicit dependency semantics with same-identity duplicate no-op and different-identity duplicate conflict rejection
- Commit boundary: `feat(task): add dependency coordination`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-008 Task Completion, Failure, And Evidence
- Objective: define completion, failure, and evidence domain contracts
- Authoritative specification: `docs/specifications/K7.8-task-completion-failure-and-evidence.md`
- Allowed production files: additive task-domain files, `lib.rs`, `errors.rs`
- Likely public API surface: completion result, failure code, failure category, failure reason, failure reference, evidence identity and reference types, recovery reference if required
- Explicit non-goals: execution recovery, retry scheduling, storage
- Upstream compatibility obligations: K5 event compatibility by reference only, K6 failure semantics not redefined
- Test groups: `task_completion_failure_evidence_*`
- Validation gates: full repository gates
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: explicit completion, failure, and evidence validation with deterministic distinction between failure, rejection, and dependency blockage
- Commit boundary: `feat(task): add completion failure and evidence model`
- Current status: `IMPLEMENTED — REVIEW PASSED`

### K7-009 Task Integration And Conformance
- Objective: add cross-layer compatibility and conformance coverage
- Authoritative specification: `docs/specifications/K7.9-task-integration-and-conformance.md`
- Allowed production files: K7 domain files only if non-breaking additions are still required; otherwise tests and docs only
- Likely public API surface: none required unless additive data-only compatibility helpers are justified
- Explicit non-goals: event bus, publishing, workflow side effects, execution, persistence, async runtime
- Upstream compatibility obligations: K1-K6 frozen APIs remain usable and unchanged
- Test groups: `task_integration_conformance_*`, compatibility suites for K1-K6 reuse, separation tests for assignment vs lifecycle, readiness vs lifecycle, failure vs rejection, dependency satisfaction vs lifecycle mutation
- Validation gates: full repository gates plus host validation requirement
- Traceability requirement: architecture traceability plus pending CES mapping
- Completion criteria: compatibility and conformance evidence complete and all prior K7 slices pass without lifecycle or readiness conflation
- Commit boundary: `test(task): add integration and conformance coverage`
- Current status: `IMPLEMENTED — REVIEW PASSED`

## Final Determination

K7 implementation is complete. The architecture review remains passed, K7-001 through K7-009 are implemented, public API inventory is recorded, compatibility is preserved, and native verification remains blocked by the missing linker environment on Saturday, July 18, 2026.
