# CHANGELOG

## Status
Draft

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-18

## Applies To
CHELA-X Kernel repository history.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## K8 Execution Engine

### Added

- Additive `ExecutionSessionId`, `ExecutionRequest`, `ExecutionContext`, `ExecutionSession`, `ExecutionOutcome`, `ExecutionTermination`, `ExecutionEvidenceBinding`, `ExecutionRetryEligibilityDecision`, and `ExecutionAuditReference`
- Deterministic K8 execution validation and conformance coverage over request, context, session, outcome, retry eligibility, and architecture boundaries
- K8 documentation closure, native verification record, traceability closure, backlog closure, and API freeze for next-milestone consumption

### Validation

- Authoritative host-native verification passed with `790 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`, exit code `0`
- Compile validation passed for `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo check --workspace --all-features --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-features --all-targets -- -D warnings`, `cargo doc --workspace --no-deps`, `cargo test --doc`, and `git diff --check`
- Historical Codex linker isolation remains non-authoritative environment context only

### Boundaries

- K8 public API is additive only
- No architecture change
- No scheduler, worker, queue, transport, filesystem, network, or database infrastructure inside `kernel-domain`
- No lifecycle mutation introduced
- No ADR required

## K7 Task Engine

### Added

- Additive K7-001 task-foundation identities for task definitions, task instances, task dependencies, and task evidence
- Additive K7-001 task reference vocabulary for definitions, instances, dependencies, evidence, workflow, and step bindings
- Immutable K7-002 task-definition model covering version, name, description, kind, input contracts, output contracts, requirements, evidence requirements, completion requirements, optional failure-policy reference, and optional workflow or step bindings
- Immutable K7-003 task-instance model covering definition snapshot binding, explicit creation context, validated input and output bindings, optional workflow and step bindings, and explicit initial `Pending` state representation
- Deterministic K7-004 task-ownership and task-assignment model covering explicit accountable owner binding, explicit assignee binding, authority-scoped assignment requests, no-op or rejection outcomes, and K3 or K4 fact consumption without execution behavior
- Deterministic K7-005 task-priority and task-readiness model covering immutable task-priority binding, validated explicit priority class and value construction, pure readiness evaluation over explicit lifecycle, ownership, assignment, authorization, dependency, and evidence facts, and stable blocked or rejected outcomes without scheduler semantics
- Deterministic K7-006 task-lifecycle and task-state model covering immutable lifecycle snapshots, sequence-aware transition requests, canonical transition map validation, readiness-gated start semantics, explicit completion and failure guard facts, and pure allowed, rejected, or no-op outcomes without runtime orchestration
- Deterministic K7-007 task-dependency coordination model covering explicit dependency graph references, typed source and target edges, canonical completion, success, evidence, and output dependency rules, deterministic duplicate and cycle validation, and aggregate satisfied, unsatisfied, unresolved, or rejected coordination outcomes without runtime orchestration
- Deterministic K7-008 task-completion, task-failure, and task-evidence model covering explicit completion results, typed output references and bindings, infrastructure-neutral evidence identity and metadata, stable failure references and recovery references, deterministic validation outcomes, and distinct completed, failed, or rejected outcome vocabulary without runtime execution or storage
- Deterministic K7-009 integration and conformance coverage covering explicit definition-to-instance composition, dependency-to-readiness composition, readiness-to-lifecycle composition, completion-to-lifecycle composition, failure-to-lifecycle composition, identity continuity, state-sequence continuity, deterministic full-flow evaluation, and cross-concern separation without introducing a runtime facade
- Deterministic K7-001 unit coverage for immutable value semantics, identity separation, and reference preservation
- Deterministic K7-002 unit coverage for definition construction, duplicate rejection, workflow-binding validation, field preservation, and immutable ordering
- Deterministic K7-003 unit coverage for definition snapshot preservation, input validation, workflow and step binding validation, explicit initial state, and immutable instance creation
- Deterministic K7-004 unit coverage for ownership preservation, assignment no-op or rejection outcomes, reassignment history preservation, explicit authority handling, and lifecycle separation
- Deterministic K7-005 unit coverage for priority ordering, readiness blockers, readiness-versus-lifecycle separation, readiness-versus-assignment separation, contradiction rejection, and non-scheduler guarantees
- Deterministic K7-006 unit coverage for every approved lifecycle edge, sequence mismatch rejection, terminal-state protection, no-op semantics, readiness integration, and separation from assignment, ownership, priority, and runtime concerns
- Deterministic K7-007 unit coverage for dependency construction, duplicate and cycle validation, satisfaction and unresolved outcomes, aggregate coordination, and separation from lifecycle mutation, readiness mutation, assignment, dispatch, and scheduler concerns
- Deterministic K7-008 unit coverage for completion validation, failure validation, evidence identity and declaration validation, completion-versus-failure conflict rejection, and separation from lifecycle mutation, retry, and runtime infrastructure
- Deterministic K7-009 integration coverage for happy-path completion and failure flows, blocked readiness rejection, dependency-readiness composition, lifecycle guard composition, mutual exclusion, immutability, determinism, and additive compatibility preservation

### Validation

- K7 implementation is complete through `K7-009`
- Architecture remains unchanged
- ADR not required
- Native `cargo test --workspace --all-targets` passed on the primary machine on Saturday, July 18, 2026 with `765 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`
- Defect-fix history recorded for `e7f8256`, `8bf4390`, and `c2e8a36`

### Boundaries

- No task execution
- No scheduler
- No persistence
- No async runtime
- No network
- No implicit event publication

## K6 Workflow Engine

### Added

- Workflow engine foundation types and additive workflow reference surface
- Immutable workflow definitions and workflow instances
- Deterministic workflow transition control composed over frozen K2 lifecycle semantics
- Deterministic workflow step coordination
- Canonical workflow authorization integration over existing K3 authorization facts
- Canonical workflow event integration over existing K5 event-envelope contracts
- Deterministic workflow failure, retry, and recovery decisions
- K6 traceability, validation closure, API documentation, and API freeze documentation

### Validation

- 595 unit tests passed
- 0 tests failed
- Formatting, checking, Clippy, documentation, and doc-test gates passed

### Boundaries

- Architecture freeze preserved
- No workflow execution runtime
- No scheduler
- No persistence
- No event bus
- No publishing
- No worker or executor support

## K5 Enterprise Event System

### Added

- Canonical `EventStreamId` and `EventStream`
- Canonical `EventSequence` and `StreamPosition`
- Canonical `StreamAppendCandidate` and `validate_stream_append`
- Canonical `EventReplayStart`, `EventReplayRequest`, and `EventReplaySource`
- Canonical `EventReplayEntry`
- Canonical `validate_replay_ordering` and `validate_event_replay`

### Validation

- 382 unit tests passed
- 0 tests failed
- Formatting, checking, Clippy, documentation, and doc-test gates passed

### Boundaries

- Architecture freeze preserved
- K6 Runtime Event Bus is next
- No Event Bus
- No Event Store
- No transport
- No publishing
- No replay execution engine

## K5.2 Event Validation

### Added

- Canonical `validate_event_envelope`
- Canonical `validate_event_identity`
- Canonical `validate_event_version`
- Canonical `validate_event_timestamps`
- Canonical `validate_event_payload`
- Canonical `validate_event_integrity`

### Validation

- 304 unit tests passed
- 0 tests failed
- Formatting, checking, Clippy, documentation, and doc-test gates passed

### Boundaries

- K5.3 Event Streams is next
- K5-017 through K5-022 remain TODO
- No Event Bus
- No Event Store
- No transport
- No publishing
- No replay execution

## K5.1 Canonical Event Envelope

### Added

- Canonical `EventId`
- Canonical `EventType`
- Canonical `EventVersion`
- Canonical `EventClassification`
- Canonical `CorrelationId`
- Canonical `EventCausation`
- Canonical `EventComponent` and `EventSource`
- Canonical `EventSubjectType`, `EventSubjectId`, and `EventSubject`
- Canonical `EventActorId`, `EventTraceReference`, and `EventTrace`
- Generic canonical `EventEnvelope<P>`

### Validation

- 236 unit tests passed
- 0 tests failed
- Formatting, checking, Clippy, documentation, and doc-test gates passed

### Boundaries

- No Event Bus
- No Event Store
- No transport
- No publishing
- No replay execution

## 0.5.0
- Finalized K5 Enterprise Event System with canonical host evidence of `382 passed` and `0 failed`.
- Marked K5-017 through K5-022 as `PASS`, K5.3 Event Streams as `PASS`, K5.4 Replay as `PASS`, and K6 Runtime Event Bus as `NEXT`.
- Finalized K5.2 event validation with canonical host evidence of `304 passed` and `0 failed`.
- Marked K5-011 through K5-016 as `PASS` and K5.3 Event Streams as `NEXT` while preserving K5-017 through K5-022 as `TODO`.
- Finalized K3 and K4.1 canonical host validation evidence as `108 passed`, `0 failed`, and `0 ignored`.
- Added additive K4.2 runtime lifecycle control primitives for validated heartbeat ingestion, heartbeat freshness classification, lease assessment and renewal, deterministic runtime health assessment, immutable runtime state snapshots, recovery eligibility, and pure supervisor outcomes.
- Added validated registry operations that apply heartbeat, lease, presence, and supervisor updates atomically without introducing runtime execution, scheduling, or background workers.
- Added K4.2 CES-traceable tests for heartbeat, freshness, lease, health, supervisor, and registry consistency invariants.
- Corrected K4.2 validation precedence so retired runtimes reject heartbeat and lease renewal before later freshness, duplicate, sequence, or renewal-window checks can mask the terminal outcome.
- Corrected lease-renewal duplicate classification so an already-applied renewal returns `LeaseDuplicate`, while a different lower-sequence operation still returns `LeaseSequenceRegression`.
- Recorded K4.2 sandbox validation evidence: `cargo fmt`, `cargo check`, `cargo clippy`, `cargo doc`, and `cargo test --doc` pass; native `cargo test --all-targets` remains blocked in this environment because linker `cc` is unavailable.

## 0.4.0
- Corrected K3 authorization enforcement so permit policies no longer authorize requests without a matching grant scope, and deny decision construction now preserves governing policy references for explicit and non-policy denials.
- Added K3 regression coverage for explicit-deny evidence retention, non-policy denial decision construction, deterministic permission-scope rejection, and evidence-presence enforcement.
- Added additive `kernel-domain::runtime` primitives for runtime identity, capability descriptors, lease records, heartbeat records, presence states, runtime health, immutable agent registrations, and deterministic in-memory agent registry indexing.
- Preserved the frozen K1-K3 surface while adding non-breaking runtime identifiers and additive agent getters needed by the registry layer.
- Added K4.1 CES-traceable tests for deterministic registration, duplicate detection, runtime and capability lookup, presence transitions, deregistration, lease validation, and heartbeat updates.
- Recorded K4.1 sandbox validation evidence: `cargo fmt`, `cargo check`, `cargo clippy`, `cargo doc`, and `cargo test --doc` pass; native `cargo test --all-targets` remains blocked in this environment because linker `cc` is unavailable.

## 0.3.0
- Added additive `kernel-domain::enforcement` primitives for deterministic authorization evaluation, policy or grant evidence resolution, authority checks, separation-of-duties checks, delegation bounds, evaluation traces, and decision construction.
- Preserved the frozen K1/K2 public surface while adding non-breaking getters and enforcement-specific input spec types.
- Added K3 CES-traceable tests for identity, scope, permission, explicit deny, authority, separation of duties, delegation bounds, deterministic outcomes, and trace order.
- Recorded K3 sandbox validation evidence: `cargo fmt`, `cargo check`, `cargo clippy`, `cargo doc`, and `cargo test --doc` passed; native `cargo test --all-targets` remains blocked in this environment because linker `cc` is unavailable.

## 0.2.5
- Finalized K2 canonical host validation with `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, `cargo doc`, `cargo test --doc`, and `git diff --check` all passing.
- Recorded canonical host unit-test evidence as `58 passed`, `0 failed`, `0 ignored`.
- Promoted supported K2 traceability entries from `IMPLEMENTED` to `VERIFIED`.
- Marked K2 complete while preserving that K3 has not started.

## 0.2.4
- Clarified the repository-local validation authority policy for host, approved CI, and Codex sandbox evidence.
- Corrected K2 status wording to `PASS WITH HOST VALIDATION PENDING` instead of treating the Codex sandbox linker limitation as a project blocker.
- Recorded the expected K2 test baseline of `58` tests as pending canonical host execution evidence.

## 0.2.3
- Added the additive K2 `kernel-domain::state` module for deterministic state snapshots, transition requests, transition outcomes, lifecycle guards, and workflow failure codes.
- Enforced CES-traced lifecycle validation for enterprise, workspace, project, organizational-unit, ownership, human, agent, decision, delegation, and workflow state changes.
- Added K2 traceability, API, implementation-plan, and validation documentation, including deferred-semantics notes where CES does not define a resume path.

## 0.2.2
- Accepted verified host validation evidence for K1.1 after the Codex sandbox could not access `/usr/bin/cc`, `/usr/bin/gcc`, or `/usr/bin/cargo` by absolute path.
- Recorded K1.1 validation as `PASS` with `38 passed`, `0 failed`, and `0 ignored` unit tests.
- Updated traceability evidence to use validated host results instead of the earlier sandbox linker blocker wording.
- Finalized the K1 domain API freeze as `FROZEN FOR K2 CONSUMPTION` and marked Kernel as ready for K2.

## 0.2.1
- Installed a stable Rust toolchain through `rustup` and verified direct toolchain execution.
- Completed K1.1 domain API corrections, including authorization evaluation-order, policy, workflow, and agent failure or recovery reference primitives.
- Replaced wide constructor signatures with validated spec structs for `AgentDefinition`, `DecisionRecord`, and `DelegationReference`.
- Froze the K1 public API baseline in `docs/API.md` and `docs/API-FREEZE.md`.
- Recorded initial K1.1 validation evidence before final host-verified gate closure.

## 0.2.0
- Replaced `kernel-bootstrap` with std-only `kernel-domain`.
- Added strongly typed identifiers for enterprise, ownership, authorization, decision, policy, workflow, and delegation records.
- Added ownership, identity, lifecycle, request, decision, authorization, agent, and delegation domain primitives.
- Added unit tests for identifier, ownership, lifecycle, request, decision, and delegation validation rules.
- Recorded K1 traceability and validation state, including the current Rust toolchain environment blocker.

## 0.1.0
- Initialized CHELA-X Kernel repository baseline.
- Added bootstrap documentation, traceability framework, and validation guidance.
- Added minimal Rust workspace scaffolding without business logic.

## K5.1 Canonical Event Envelope

### Status

PASS

### Added

- Canonical `EventId`
- Canonical `EventType`
- Canonical `EventVersion`
- Canonical `EventClassification`
- Canonical `CorrelationId`
- Canonical `EventCausation`
- Canonical `EventComponent`
- Canonical `EventSource`
- Canonical `EventSubjectType`
- Canonical `EventSubjectId`
- Canonical `EventSubject`
- Canonical `EventActorId`
- Canonical `EventTraceReference`
- Canonical `EventTrace`
- Generic `EventEnvelope<P>`

### Engineering Gates

- cargo fmt — PASS
- cargo check — PASS
- cargo test — PASS (236 passed, 0 failed)
- cargo clippy — PASS
- cargo doc — PASS
- cargo test --doc — PASS

### Notes

The implementation remains domain-only.

No Event Bus, persistence, transport, replay, scheduling, or runtime execution has been introduced.
