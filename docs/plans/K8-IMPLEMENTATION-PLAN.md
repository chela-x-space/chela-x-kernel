# K8 Implementation Plan

## Status
Implementation Complete

## Last Updated
2026-07-18

## Exact K8 Title
`K8 Execution Engine`

## Purpose
Plan the bounded K8 execution-domain milestone that consumes frozen K1-K7 contracts additively without introducing runtime infrastructure or changing frozen public APIs.

## Authoritative CES Sources
- `docs/kernel-architecture/01-kernel-overview.md`
- `docs/kernel-architecture/09-execution-architecture.md`
- `docs/kernel-architecture/15-roadmap.md`
- `docs/kernel-architecture/16-traceability.md`
- `docs/K4.2-RUNTIME-SUPERVISION.md`
- `docs/specifications/K6.5-workflow-step-coordination.md`
- `docs/specifications/K6.8-workflow-failure-and-recovery.md`
- `docs/specifications/K7.8-task-completion-failure-and-evidence.md`
- `docs/specifications/K7.9-task-integration-and-conformance.md`

## Scope
- Execution request contracts over approved work
- Immutable execution context
- Immutable execution session identity and audit snapshot
- Explicit execution outcomes and termination vocabulary
- Explicit execution-evidence bindings
- Deterministic retry-eligibility decisions
- Reference-only compatibility with events and memory
- Cross-concern conformance with runtime, workflow, and task APIs

## Non-Goals
- Worker dispatch
- Scheduler or queue semantics
- Process spawning
- Network transport
- Filesystem or database access
- Event publication
- Memory persistence
- Automatic lifecycle mutation
- Automatic retry or timeout execution

## Frozen Dependencies From K1-K7
- K1 identifiers and value primitives
- K2 `StateSequence`, transition references, and lifecycle validation conventions
- K3 authorization decisions, principals, and evidence references
- K4 runtime identity, capability, lease, supervision, and snapshot facts
- K5 event-envelope, event type, subject, source, and trace references
- K6 workflow definition, instance, step, authorization, event, and failure or recovery contracts
- K7 task identity, instance, ownership, assignment, readiness, lifecycle, dependency, completion, failure, and evidence contracts

## Implemented Domain Contracts
- `ExecutionSessionId`
- `ExecutionRequest`
- `ExecutionContext`
- `ExecutionSession`
- `ExecutionOutcome`
- `ExecutionTermination`
- `ExecutionEvidenceBinding`
- `ExecutionRetryEligibilityDecision`
- `ExecutionAuditReference`

## Implemented Public API Surface
- New additive `execution` module under `crates/kernel-domain/src`
- Additive `kernel-domain` re-exports for approved K8 execution types only
- No runtime facade, repository, scheduler, or executor API

## Invariants
- Execution consumes approved tasks and never defines task meaning
- One execution session represents one governed attempt
- Execution context is immutable within one session
- Outcomes are explicit and mutually exclusive
- Retry eligibility is explicit and never automatic
- Evidence is preserved by reference and never silently discarded

## Allowed Operations
- Construct validated execution requests
- Construct immutable execution contexts
- Create immutable execution-session snapshots
- Evaluate execution outcomes from explicit supplied facts
- Evaluate retry eligibility from explicit supplied facts
- Bind execution evidence and audit references

## Rejection Conditions
- Missing approved task binding
- Missing runtime identity or incompatible runtime fact
- Missing authorization or delegation fact where required
- Missing required execution evidence
- Contradictory outcome or termination facts
- Duplicate or structurally invalid execution references
- Any request that implies hidden side effects or runtime lookup

## Identity Rules
- Execution session identity is distinct from task, workflow, runtime, event, and evidence identity
- Execution contracts reuse canonical references instead of raw strings
- Task, workflow, runtime, authorization, and evidence identities must remain continuous across execution records

## Validation Rules
- Constructors validate structure and identity continuity
- Equivalent inputs produce equivalent decisions
- All time values must be caller-supplied `TimeReference`
- No execution control may call the clock, network, filesystem, or database

## Lifecycle Interaction
- K8 consumes explicit K7 lifecycle facts only
- K8 does not mutate `TaskState`, `TaskStateSnapshot`, or `TaskTransitionDecision`
- Any future task-state transition remains an explicit downstream composition step

## Authorization Interaction
- K8 consumes K3 authorization outcomes by reference only
- K8 does not evaluate policy or authorization engines

## Delegation Interaction
- K8 consumes delegation and authority references by reference only
- K8 does not redefine delegation scope or separation-of-duties semantics

## Workflow Interaction
- K8 consumes workflow bindings and workflow-instance references by reference only
- K8 does not orchestrate workflow stages or redefine workflow transitions

## Task Interaction
- K8 consumes frozen K7 task readiness, lifecycle, dependency, completion, failure, and evidence facts
- K8 does not redefine ownership, assignment, priority, readiness, dependency, or task outcome semantics

## Determinism Rules
- No hidden clock access
- No randomness
- No global mutable state
- No implicit retries, recovery, or dispatch

## Immutability Rules
- No public mutable fields
- No `&mut self` public mutation API
- Accepted operations return new immutable values only

## Concern Separation
- Runtime supervision remains K4
- Event publication remains outside K8
- Memory retention remains K9
- Transport and execution infrastructure remain out of scope

## Native Test Matrix
| Requirement | Native test coverage |
| --- | --- |
| `K8-001` | execution-request construction, task binding, and identity preservation |
| `K8-002` | execution-context immutability, supplied-time usage, and authorization reference preservation |
| `K8-003` | execution-session identity, attempt separation, and audit snapshot preservation |
| `K8-004` | outcome and termination exclusivity, explicit succeeded or failed or cancelled vocabulary |
| `K8-005` | evidence binding completeness, duplicate rejection, and reference continuity |
| `K8-006` | retry-eligibility determinism, policy-reference consumption, and no automatic retry |
| `K8-007` | event and memory compatibility by reference without publication or persistence |
| `K8-008` | cross-concern conformance proving no lifecycle mutation, no dispatch, and no runtime lookup |

## Compile-Gate Matrix
| Gate | Actual result |
| --- | --- |
| `cargo fmt --all -- --check` | `PASS` |
| `cargo check --workspace --all-targets` | `PASS` |
| `cargo check --workspace --all-features --all-targets` | `PASS` |
| `cargo clippy --workspace --all-targets -- -D warnings` | `PASS` |
| `cargo clippy --workspace --all-features --all-targets -- -D warnings` | `PASS` |
| `cargo doc --workspace --no-deps` | `PASS` |
| `cargo test --doc` | `PASS` |

## Static-Audit Matrix
| Audit | Actual result |
| --- | --- |
| runtime or infrastructure keywords | `PASS` |
| clock and randomness usage | `PASS` |
| mutable public API | `PASS` |
| cross-concern mutation API | `PASS` |

## Traceability Table
| Kernel requirement | Repository-local source | Supporting CES-traceable source | Implemented contract or type | Dependency | Validation method | Test category | Classification |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `K8-001` | execution request must bind approved work | `09-execution-architecture.md` §5, `15-roadmap.md` §8 | `CES-B0-030.18` via `K6.3`, `K7.9` deferred execution boundary | `ExecutionRequest` | K7 task instance, readiness, lifecycle references | native tests | construction and binding | `NATIVE_TESTED` |
| `K8-002` | execution context is explicit and immutable | `09-execution-architecture.md` §6 | `CES-B0-027.10`, `CES-B0-027.21` via `K4.2`; `CES-B0-030.17` via `K6.8` | `ExecutionContext` | K3 authorization, K4 runtime, K7 evidence | native tests | context invariants | `NATIVE_TESTED` |
| `K8-003` | one session equals one governed attempt | `09-execution-architecture.md` §7 | `CES-B0-030.18` via `K6.3` | `ExecutionSession`, `ExecutionSessionId` | K1 identifiers, K7 task references | native tests | session identity | `NATIVE_TESTED` |
| `K8-004` | outcomes and termination are explicit | `09-execution-architecture.md` §8, §14 | `CES-B0-030.13`, `CES-B0-030.18` via `K6.4`, `K6.8` | `ExecutionOutcome`, `ExecutionTermination` | K7 completion and failure outcomes | native tests | outcome and termination | `NATIVE_TESTED` |
| `K8-005` | evidence is preserved and never silently discarded | `09-execution-architecture.md` §9 | `CES-B0-030.17` via `K6.3`, `K6.8` | `ExecutionEvidenceBinding` | K5 event evidence references, K7 task evidence | native tests | evidence coverage | `NATIVE_TESTED` |
| `K8-006` | retry eligibility is explicit and deterministic | `09-execution-architecture.md` §10 | `CES-B0-030.14`, `CES-B0-030.18` via `K6.5`, `K6.8` | `ExecutionRetryEligibilityDecision` | K4 recovery facts, K7 failure policy references | native tests | retry eligibility | `NATIVE_TESTED` |
| `K8-007` | execution outcomes compose with events and memory by reference only | `01-kernel-overview.md` §4, `09-execution-architecture.md` §12-§13 | K5 event traceability and K9 memory deferral | `ExecutionAuditReference` | K5 event references, future K9 memory references | compile gates | composition only | `COMPILE_GATED` |
| `K8-008` | K8 preserves architecture boundaries and concern separation | `01-kernel-overview.md` §6-§9, `16-traceability.md` §7 | runtime, workflow, task, and execution separation obligations | conformance coverage only | K1-K7 frozen APIs | static audit | boundary enforcement | `STATIC_AUDIT` |

## Implemented File Sequence
1. `crates/kernel-domain/src/execution.rs`
2. `crates/kernel-domain/src/execution_request.rs`
3. `crates/kernel-domain/src/execution_context.rs`
4. `crates/kernel-domain/src/execution_session.rs`
5. `crates/kernel-domain/src/execution_outcome.rs`
6. `crates/kernel-domain/src/execution_retry.rs`
7. `crates/kernel-domain/src/execution_validation.rs`
8. `crates/kernel-domain/src/lib.rs`
9. `crates/kernel-domain/src/execution_request_tests.rs`
10. `crates/kernel-domain/src/execution_context_tests.rs`
11. `crates/kernel-domain/src/execution_session_tests.rs`
12. `crates/kernel-domain/src/execution_outcome_tests.rs`
13. `crates/kernel-domain/src/execution_retry_tests.rs`
14. `crates/kernel-domain/src/execution_conformance_tests.rs`
15. `crates/kernel-domain/src/execution_separation_tests.rs`
16. `crates/kernel-domain/src/lib.rs`
17. `crates/kernel-domain/src/errors.rs`
18. `docs/plans/K8-IMPLEMENTATION-PLAN.md`
19. `docs/backlog/K8-BACKLOG.md`
20. `docs/IMPLEMENTATION-PLAN.md`
21. `docs/TRACEABILITY.md`
22. `docs/VALIDATION.md`

## Commit Sequence
1. `feat(execution): add K8 execution engine contracts`

## Review Gates
- Frozen K1-K7 public APIs unchanged
- No runtime infrastructure introduced
- CES mapping remains repository-local and non-fabricated
- Native tests required on the primary machine before any K8 API freeze

## Completion Criteria
- K8 specification package approved
- Architecture review passed
- Planned K8 requirements implemented additively
- Compile validation passed
- Primary-host native rerun still required
- K8 API not yet frozen
- No ADR required unless scope changes

## Deferred Work
- Event publication policy and event-store integration
- Memory retention and storage contracts
- Timeout execution semantics beyond explicit supplied facts
- Operational dispatch, transport, scheduler, or worker infrastructure

## Architecture And ADR Assessment
- Architecture fit: `PASS — NO ADR REQUIRED`
- Current K8 architecture review status: `PASSED`
- K8 implementation status: `COMPLETE`
- K8 compile validation status: `PASSED`
- K8 native verification status: `BLOCKED — PRIMARY HOST RERUN REQUIRED`
- K8 API status: `NOT YET FROZEN`
- CES requirement mapping beyond repository-local inherited sources remains pending the authoritative K8 specification package and must not be fabricated
