# K7 Task Engine Backlog

## K7 Milestone
PLANNING

## Specification Package
ARCHITECTURE APPROVED

## Architecture Review
PASS

## Implementation
NOT STARTED

## API
NOT ESTABLISHED

## Backlog Items

### K7-001 Task Engine Foundation
- Status: `READY FOR IMPLEMENTATION`
- Purpose: define stable task vocabulary, identity, references, and foundational validation expectations
- Source specification: `docs/specifications/K7.1-task-engine-foundation.md`
- Dependencies: architecture review of K7 specification package
- Implementation scope: `TaskDefinitionId`, `TaskInstanceId`, `TaskDependencyId`, `TaskEvidenceId`, explicit reference primitives only where needed, foundational validation outcomes, additive error surface if required
- Prohibited scope: execution, scheduler, async runtime, persistence, event bus, networking
- Expected files: additive task-domain Rust files under `crates/kernel-domain/src`, `lib.rs`, `errors.rs`
- Required tests: `task_engine_foundation_*`, compatibility tests for consumed K1-K6 references
- Required documentation updates: traceability and API notes only if needed during implementation
- Validation commands: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo test --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo doc --workspace --no-deps`, `cargo test --doc`, `git diff --check`, `git status --short`
- Commit message recommendation: `feat(task): add task engine foundation`
- Acceptance checklist:
  - stable task identity defined
  - no public `TaskId`
  - identity immutable
  - no runtime resolution
  - no infrastructure-bearing fields
  - additive compatibility preserved

### K7-002 Task Definition
- Status: `BLOCKED BY K7-001`
- Purpose: define immutable task-definition model
- Source specification: `docs/specifications/K7.2-task-definition.md`
- Dependencies: K7-001
- Implementation scope: task-definition identity, version, name, contracts, requirements, evidence requirements, failure-policy references
- Prohibited scope: live runtime state, executor state, scheduler semantics
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_definition_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add task definition model`
- Acceptance checklist:
  - immutable definition
  - explicit input and output contracts
  - explicit completion requirements
  - duplicate and contradiction rejection
  - workflow compatibility preserved

### K7-003 Task Instance
- Status: `BLOCKED BY K7-002`
- Purpose: define immutable task-instance model
- Source specification: `docs/specifications/K7.3-task-instance.md`
- Dependencies: K7-002
- Implementation scope: task-instance identity, definition snapshot binding, creation context, workflow and step bindings, initial state
- Prohibited scope: execution, persistence, implicit events
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_instance_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add task instance model`
- Acceptance checklist:
  - explicit instance identity
  - immutable definition snapshot
  - deterministic creation
  - no execution side effect
  - workflow binding additive only

### K7-004 Task Ownership And Assignment
- Status: `BLOCKED BY K7-003`
- Purpose: separate accountable ownership from future-execution assignment
- Source specification: `docs/specifications/K7.4-task-ownership-and-assignment.md`
- Dependencies: K7-003
- Implementation scope: ownership, assignment, authority, assignment status, evidence, reassignment outcomes
- Prohibited scope: policy redefinition, execution, capacity reservation
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_ownership_assignment_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add ownership and assignment model`
- Acceptance checklist:
  - owner distinct from assignee
  - `Assigned` used only as assignment status
  - eligibility validation consumes K3 and K4 facts only
  - deterministic reassignment outcomes
  - no execution side effect
  - no unsupported multi-assignee behavior

### K7-005 Task Priority And Readiness
- Status: `BLOCKED BY K7-004`
- Purpose: define explicit priority and side-effect-free readiness evaluation
- Source specification: `docs/specifications/K7.5-task-priority-and-readiness.md`
- Dependencies: K7-004
- Implementation scope: priority classes, readiness decisions, blocker categories, readiness evidence
- Prohibited scope: scheduler, resource reservation, worker dispatch
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_priority_readiness_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add priority and readiness validation`
- Acceptance checklist:
  - priority not treated as scheduler instruction
  - `Ready` and `Blocked` remain derived readiness outcomes
  - readiness blocked reasons stable
  - deterministic ordering
  - no mutation of workflow state
  - no agent-capacity claims

### K7-006 Task Lifecycle And State
- Status: `BLOCKED BY K7-005`
- Purpose: define canonical task lifecycle and deterministic transition control
- Source specification: `docs/specifications/K7.6-task-lifecycle-and-state.md`
- Dependencies: K7-005
- Implementation scope: task-state vocabulary, snapshots, transitions, reasons, evidence, sequence rules
- Prohibited scope: workflow-state redesign, persistence, implicit event emission
- Expected files: additive task-domain Rust files, possibly `state.rs` if approved, `lib.rs`, `errors.rs`
- Required tests: `task_lifecycle_state_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add lifecycle and transition control`
- Acceptance checklist:
  - canonical lifecycle frozen to `Pending`, `InProgress`, `Completed`, `Failed`, `Cancelled`, `Archived`
  - legal and illegal edges explicit
  - terminal-state protection
  - deterministic no-op handling
  - K2 workflow semantics unchanged

### K7-007 Task Dependency Coordination
- Status: `BLOCKED BY K7-006`
- Purpose: define deterministic task-to-task dependency coordination
- Source specification: `docs/specifications/K7.7-task-dependency-coordination.md`
- Dependencies: K7-006
- Implementation scope: dependency types, validation, cycle detection where possible, readiness contribution
- Prohibited scope: scheduler, automatic downstream execution, automatic state mutation
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_dependency_coordination_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add dependency coordination`
- Acceptance checklist:
  - explicit direction
  - self-dependency rejected
  - duplicate same identity yields no-op
  - duplicate different identity yields conflict rejection
  - cycle rejection where detectable
  - workflow-step ordering unchanged

### K7-008 Task Completion, Failure, And Evidence
- Status: `BLOCKED BY K7-007`
- Purpose: define explicit completion, failure, and evidence semantics
- Source specification: `docs/specifications/K7.8-task-completion-failure-and-evidence.md`
- Dependencies: K7-007
- Implementation scope: completion, output, failure code or category, evidence identity and metadata, recovery reference only if required
- Prohibited scope: execution recovery, retry scheduling, storage infrastructure
- Expected files: additive task-domain Rust files, `lib.rs`, `errors.rs`
- Required tests: `task_completion_failure_evidence_*`
- Required documentation updates: API and traceability coverage after implementation
- Validation commands: full repository gates
- Commit message recommendation: `feat(task): add completion failure and evidence model`
- Acceptance checklist:
  - explicit completion
  - explicit failure code, category, reason, and reference vocabulary
  - evidence stable identity
  - duplicate evidence rejection
  - rejection distinct from declared failure

### K7-009 Task Integration And Conformance
- Status: `BLOCKED BY K7-008`
- Purpose: prove K1-K6 compatibility and K7 conformance
- Source specification: `docs/specifications/K7.9-task-integration-and-conformance.md`
- Dependencies: K7-008
- Implementation scope: compatibility tests, cross-module domain tests where approved, conformance evidence
- Prohibited scope: event bus, publishing, execution, persistence, async runtime, workflow side effects
- Expected files: task test modules, additive docs if needed, no infrastructure modules
- Required tests: `task_integration_conformance_*`, compatibility tests for K1-K6 consumed APIs
- Required tests: `task_integration_conformance_*`, compatibility tests for K1-K6 consumed APIs, `assignment_vs_lifecycle_*`, `readiness_vs_lifecycle_*`, `failure_vs_rejection_*`, `dependency_satisfaction_vs_lifecycle_mutation_*`
- Required documentation updates: validation and traceability evidence as implementation progresses
- Validation commands: full repository gates plus host validation
- Commit message recommendation: `test(task): add integration and conformance coverage`
- Acceptance checklist:
  - frozen upstream APIs remain usable
  - K1-K6 tests continue to pass
  - assignment and lifecycle remain distinct
  - readiness and lifecycle remain distinct
  - dependency satisfaction does not mutate lifecycle
  - K7 compatibility evidence complete
  - no side effects introduced
  - host validation targeted

### K7-010 Documentation Closure And API Freeze
- Status: `BLOCKED BY K7-009`
- Purpose: close K7 validation and freeze the K7 public API after implementation and host validation
- Source specification: `docs/specifications/K7.9-task-integration-and-conformance.md`
- Dependencies: K7-009
- Implementation scope: documentation-only closure and API freeze
- Prohibited scope: new task behavior, runtime infrastructure, semantic redesign
- Expected files: `docs/API.md`, `docs/API-FREEZE.md`, `docs/TRACEABILITY.md`, `docs/VALIDATION.md`, `docs/IMPLEMENTATION-PLAN.md`, `docs/plans/K7-IMPLEMENTATION-PLAN.md`, `docs/backlog/K7-BACKLOG.md`, `README.md`, `CHANGELOG.md`
- Required tests: none new unless doc-test correction is required
- Required documentation updates: full K7 closure package
- Validation commands: full repository gates and authoritative host validation
- Commit message recommendation: `docs(task): close K7 validation and API freeze`
- Acceptance checklist:
  - K7 milestone closure recorded
  - downstream API-freeze declaration recorded
  - host validation recorded
  - traceability complete
  - backlog and plan closed

## Final Determination

No K7 implementation work item is marked done.
K7 backlog remains planning-only until architecture review is recorded and K7-001 implementation begins.
