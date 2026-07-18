# TRACEABILITY

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-18

## Applies To
Requirement traceability from CES and Program sources into CHELA-X Kernel, including K6 workflow-engine closure.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## K6 Milestone Summary

- Milestone: `K6 Workflow Engine`
- Status: `PASS`
- Architecture Freeze: `PRESERVED`
- Runtime baseline: `595 passed`, `0 failed`, `0 ignored`
- Public API status: `FROZEN FOR DOWNSTREAM CONSUMPTION`

## K6 Commit References

| K6 Slice | Commit | Summary |
| --- | --- | --- |
| K6-001 | `a472440` | `feat(workflow): add engine foundation` |
| K6-002 | `ea3fe77` | `feat(workflow): add canonical definition model` |
| K6-003 | `c40560c` | `feat(workflow): add canonical instance model` |
| K6-004 | `549d67d` | `feat(workflow): add deterministic transition control` |
| K6-005 | `7731f6b` | `feat(workflow): add deterministic step coordination` |
| K6-006 | `934cad1` | `feat(workflow): integrate canonical authorization decisions` |
| K6-007 | `b1e1189` | `feat(workflow): integrate canonical enterprise events` |
| K6-008 | `9b2839f` | `feat(workflow): add deterministic failure recovery control` |
| K6-009 | Working tree documentation closure | Validation, traceability, API, freeze, backlog, and milestone closure |

## K6 Requirements Matrix

| Requirement ID | Requirement Summary | Specification Source | Implementation Location | Public API Or Type | Tests Proving Requirement | Validation Status | Commit Reference | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `K6-001` | Workflow engine foundation remains additive, deterministic, and side-effect free. | `docs/specifications/K6.1-workflow-engine-foundation.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowEngineFoundation`, `WorkflowRetryLimit`, `WorkflowRetryPolicyReference`, `WorkflowRecoveryReference`, `WorkflowAuditEvidenceReference`, `WorkflowLifecycleMapReference`, workflow-related `DomainError` variants | `workflow_engine_foundation_*`, `workflow_retry_limit_rejects_zero_ces_b0_030_14`, `workflow_recovery_requires_path_reference_ces_b0_030_14` | `PASS` | `a472440` | Reuses K1 identifiers, K2 workflow state, K3 evidence, and K5 event prerequisites. |
| `K6-002` | Immutable workflow definitions bind identity, namespace, version, ownership, lifecycle map, entry steps, terminal outcomes, policy references, retry, recovery, and audit evidence. | `docs/specifications/K6.2-workflow-definition.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowDefinition`, `WorkflowStepReference`, `WorkflowTerminalOutcomeReference` | `workflow_definition_*` | `PASS` | `ea3fe77` | No definition approval or execution runtime was introduced. |
| `K6-003` | Immutable workflow instances preserve approved definition linkage, version snapshot, ownership, state snapshot, retry or recovery snapshots, and audit evidence. | `docs/specifications/K6.3-workflow-instance.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowInstance` | `workflow_instance_*` | `PASS` | `c40560c` | Reuses `WorkflowStateSnapshot`, `StableVersion`, `OwnershipPath`, and existing workflow reference types. |
| `K6-004` | Deterministic workflow transition control composes the frozen K2 lifecycle map and guards without introducing a second transition engine. | `docs/specifications/K6.4-workflow-transition-control.md`, `crates/kernel-domain/src/state.rs` | `crates/kernel-domain/src/state.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowTransitionControlRequest`, `WorkflowTransitionControl`, `WorkflowTransitionDecision`, `validate_workflow_transition` | `workflow_transition_control_*` | `PASS` | `549d67d` | K2 lifecycle semantics and guard precedence remain unchanged. |
| `K6-005` | Workflow step coordination remains declarative, ordered, immutable, and distinct from task execution. | `docs/specifications/K6.5-workflow-step-coordination.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowStepSelection`, `WorkflowStepExecutionPlan`, `WorkflowStepCoordination`, `WorkflowStepOutcomeReference` | `workflow_step_coordination_*` | `PASS` | `7731f6b` | No task runtime, dispatch, or scheduler behavior exists. |
| `K6-006` | Workflow authorization integration consumes K3 authorization facts without duplicating permission or policy semantics. | `docs/specifications/K6.6-workflow-authorization-and-policy.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowOperationReference`, `WorkflowAuthorizationContext`, `WorkflowAuthorizationRequest`, `WorkflowAuthorizationControl`, `WorkflowAuthorizationDecision` | `workflow_authorization_*` | `PASS` | `934cad1` | Reuses canonical K3 authorization decisions; no evaluation engine or identity lookup is added. |
| `K6-007` | Workflow event integration composes accepted workflow facts with canonical K5 event-envelope semantics without publishing or persisting. | `docs/specifications/K6.7-workflow-event-integration.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowEventTypeReference`, `WorkflowEventContext`, `WorkflowEventIntegrationRequest`, `WorkflowEventIntegration`, `WorkflowEventDecision` | `workflow_event_integration_*` | `PASS` | `b1e1189` | Reuses canonical K5 event types; no event bus, outbox, or publisher support exists. |
| `K6-008` | Workflow failure and recovery remain bounded, explicit, deterministic, and non-executing. | `docs/specifications/K6.8-workflow-failure-and-recovery.md` | `crates/kernel-domain/src/workflow.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `WorkflowFailureContext`, `WorkflowFailureRecord`, `WorkflowRecoveryRequest`, `WorkflowRecoveryControl`, `WorkflowRecoveryDecision` | `workflow_failure_recovery_*` | `PASS` | `9b2839f` | Reuses K2 failure and recovery semantics; no scheduler, backoff, retry queue, or workflow mutation is introduced. |
| `K6-009` | K6 closure records complete traceability, validation evidence, API documentation, freeze state, backlog closure, and compatibility evidence. | `README.md`, `CHANGELOG.md`, `docs/TRACEABILITY.md`, `docs/VALIDATION.md`, `docs/API.md`, `docs/API-FREEZE.md`, `docs/IMPLEMENTATION-PLAN.md`, `docs/plans/K6-IMPLEMENTATION-PLAN.md`, `docs/backlog/K6-BACKLOG.md` | Documentation only | Documentation-only closure over existing K6 public APIs and validation commands | Existing K6 compatibility suites: `workflow_authorization_existing_k6_001_through_k6_005_apis_remain_usable`, `workflow_event_integration_existing_k6_001_through_k6_006_apis_remain_usable`, `workflow_failure_recovery_existing_k6_001_through_k6_007_apis_remain_usable`; host validation gates | `PASS` | Working tree documentation closure | No standalone `docs/specifications/K6.9-...` file exists in the repository; closure is derived from the actual repository plans, specs, code, and host verification evidence. |

## K6 Compatibility Closure

- K1 value primitives remain reusable through `kernel-domain` re-exports in `crates/kernel-domain/src/lib.rs`.
- K2 lifecycle APIs remain reusable through `validate_workflow_transition`, `WorkflowState`, `WorkflowStateSnapshot`, and `WorkflowTransitionDecision`.
- K3 authorization semantics are consumed by reference only through `WorkflowAuthorization*`.
- K5 event-envelope semantics are consumed by composition only through `WorkflowEvent*`.
- Existing compatibility tests for K6-001 through K6-008 remain present and are part of the host-verified `595 passed` baseline.

## K6 Boundaries Confirmed

- No runtime scheduler
- No executor
- No persistence
- No event bus
- No async runtime
- No network
- No workflow mutation by step coordination, authorization integration, event integration, or recovery control

## K7 Milestone Summary

- Milestone: `K7 Task Engine`
- Current slice: `K7-009 Task Integration And Conformance`
- Status: `IMPLEMENTATION COMPLETE`
- Architecture Freeze: `PRESERVED`
- Public API status: `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

## K7 Requirements Matrix

| Requirement ID | Requirement Summary | Specification Source | Implementation Location | Public API Or Type | Tests Proving Requirement | Validation Status | Commit Reference | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `K7-001` | Foundational task identities and references remain additive, deterministic, immutable, and infrastructure-free. | `docs/specifications/K7.1-task-engine-foundation.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/task/identity.rs`, `crates/kernel-domain/src/task/reference.rs`, `crates/kernel-domain/src/task/tests.rs`, `crates/kernel-domain/src/lib.rs` | `TaskDefinitionId`, `TaskInstanceId`, `TaskDependencyId`, `TaskEvidenceId`, `TaskDefinitionReference`, `TaskInstanceReference`, `TaskDependencyReference`, `TaskEvidenceReference`, `TaskWorkflowReference`, `TaskStepReference` | `task_engine_foundation_*` | `IMPLEMENTED / VERIFIED` | `d84a83b` | `15` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-002` | Task definitions remain immutable, version-bound, ordered, deterministic, and additive to K6 workflow bindings. | `docs/specifications/K7.2-task-definition.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/definition.rs`, `crates/kernel-domain/src/task/definition_value.rs`, `crates/kernel-domain/src/task/definition_validation.rs`, `crates/kernel-domain/src/task/definition_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskDefinition`, `TaskDefinitionVersion`, `TaskDefinitionName`, `TaskDescription`, `TaskKind`, `TaskInputContract`, `TaskOutputContract`, `TaskRequirement`, `TaskCapabilityRequirement`, `TaskEvidenceRequirement`, `TaskCompletionRequirement`, `TaskFailurePolicyReference`, `DomainError::InvalidTaskDefinition` | `task_definition_*` | `IMPLEMENTED / VERIFIED` | `12c440b` | `12` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-003` | Task instances remain immutable, definition-snapshot-bound, explicitly created, and additive to K6 workflow-instance and workflow-step bindings. | `docs/specifications/K7.3-task-instance.md`, `docs/specifications/K7.6-task-lifecycle-and-state.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/instance.rs`, `crates/kernel-domain/src/task/instance_value.rs`, `crates/kernel-domain/src/task/instance_binding.rs`, `crates/kernel-domain/src/task/instance_validation.rs`, `crates/kernel-domain/src/task/instance_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskInstance`, `TaskDefinitionSnapshotReference`, `TaskCreationContext`, `TaskInputBinding`, `TaskOutputBinding`, `TaskWorkflowBinding`, `TaskStepBinding`, `TaskState`, `DomainError::InvalidTaskInstance` | `task_instance_*` | `IMPLEMENTED / VERIFIED` | `28c75ba` | `12` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-004` | Task ownership and assignment remain distinct, deterministic, authority-scoped, and additive to K3 authorization facts and K4 lifecycle facts. | `docs/specifications/K7.4-task-ownership-and-assignment.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/subject.rs`, `crates/kernel-domain/src/task/ownership.rs`, `crates/kernel-domain/src/task/assignment.rs`, `crates/kernel-domain/src/task/assignment_decision.rs`, `crates/kernel-domain/src/task/assignment_control.rs`, `crates/kernel-domain/src/task/assignment_validation.rs`, `crates/kernel-domain/src/task/ownership_tests.rs`, `crates/kernel-domain/src/task/assignment_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs` | `TaskOwner`, `TaskOwnership`, `TaskOwnershipAuthority`, `TaskOwnershipScope`, `TaskAssignee`, `TaskAssignment`, `TaskAssignmentAuthority`, `TaskAssignmentStatus`, `TaskAssignmentReason`, `TaskAssignmentRejectionReason`, `TaskAssignmentRequest`, `TaskAssignmentDecision`, `TaskAssignmentControl`, `DomainError::InvalidTaskAssignment`, `DomainError::InvalidTaskOwnership` | `task_ownership_*`, `task_assignment_*` | `IMPLEMENTED / VERIFIED` | `667512d` | `17` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-005` | Task priority remains explicit and comparable, while readiness remains a side-effect-free derived outcome over explicit ownership, assignment, authorization, dependency, and evidence facts. | `docs/specifications/K7.5-task-priority-and-readiness.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/priority.rs`, `crates/kernel-domain/src/task/readiness.rs`, `crates/kernel-domain/src/task/readiness_input.rs`, `crates/kernel-domain/src/task/readiness_decision.rs`, `crates/kernel-domain/src/task/readiness_validation.rs`, `crates/kernel-domain/src/task/priority_tests.rs`, `crates/kernel-domain/src/task/readiness_tests.rs`, `crates/kernel-domain/src/task/readiness_separation_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskPriority`, `TaskPriorityClass`, `TaskPriorityValue`, `TaskReadiness`, `TaskReadinessRequirement`, `TaskReadinessEvidence`, `TaskReadinessBlocker`, `TaskReadinessInput`, `TaskReadinessDecision`, `TaskReadinessControl`, `DomainError::InvalidTaskPriority`, `DomainError::InvalidTaskReadiness` | `task_priority_*`, `task_readiness_*` | `IMPLEMENTED / VERIFIED` | `dda16af` | `20` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-006` | Task lifecycle control remains explicit, deterministic, sequence-aware, readiness-integrated, and separated from assignment, dependency coordination, execution, and persistence concerns. | `docs/specifications/K7.6-task-lifecycle-and-state.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/lifecycle.rs`, `crates/kernel-domain/src/task/lifecycle_guard.rs`, `crates/kernel-domain/src/task/lifecycle_request.rs`, `crates/kernel-domain/src/task/lifecycle_decision.rs`, `crates/kernel-domain/src/task/lifecycle_transition.rs`, `crates/kernel-domain/src/task/lifecycle_validation.rs`, `crates/kernel-domain/src/task/lifecycle_allowed_tests.rs`, `crates/kernel-domain/src/task/lifecycle_rejected_tests.rs`, `crates/kernel-domain/src/task/lifecycle_noop_tests.rs`, `crates/kernel-domain/src/task/lifecycle_separation_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskStateSnapshot`, `TaskFailureCode`, `TaskFailureCategory`, `TaskLifecycleGuards`, `TaskTransitionRequest`, `TaskTransitionRejectionReason`, `TaskAllowedTransition`, `TaskRejectedTransition`, `TaskNoOpTransition`, `TaskTransitionDecision`, `TaskTransitionControl`, `DomainError::InvalidTaskLifecycle` | `task_lifecycle_*` | `IMPLEMENTED / VERIFIED` | `288f26c` | `25` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-007` | Task dependency coordination remains explicit, cycle-aware, deterministic, and readiness-contributing without mutating lifecycle, readiness, assignment, or execution state. | `docs/specifications/K7.7-task-dependency-coordination.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/dependency.rs`, `crates/kernel-domain/src/task/dependency_set.rs`, `crates/kernel-domain/src/task/dependency_fact.rs`, `crates/kernel-domain/src/task/dependency_decision.rs`, `crates/kernel-domain/src/task/dependency_coordination.rs`, `crates/kernel-domain/src/task/dependency_cycle.rs`, `crates/kernel-domain/src/task/dependency_evaluation.rs`, `crates/kernel-domain/src/task/dependency_validation.rs`, `crates/kernel-domain/src/task/dependency_construction_tests.rs`, `crates/kernel-domain/src/task/dependency_satisfaction_tests.rs`, `crates/kernel-domain/src/task/dependency_coordination_tests.rs`, `crates/kernel-domain/src/task/dependency_cycle_tests.rs`, `crates/kernel-domain/src/task/dependency_separation_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskDependencyGraphReference`, `TaskDependencySource`, `TaskDependencyTarget`, `TaskDependencyType`, `TaskDependencyRequirement`, `TaskDependencyStatus`, `TaskDependency`, `TaskDependencyFact`, `TaskDependencySet`, `TaskDependencyValidationRequest`, `TaskDependencyCoordinationRequest`, `TaskDependencyValidation`, `TaskDependencyDecision`, `TaskDependencyCoordinationDecision`, `TaskDependencyControl`, `DomainError::InvalidTaskDependency` | `task_dependency_*` | `IMPLEMENTED / VERIFIED` | `90cd884` | `25` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-008` | Task completion, failure, and evidence remain explicit, deterministic, identity-based, infrastructure-neutral, and distinct from lifecycle mutation, dependency blockage, authorization denial, and runtime execution semantics. | `docs/specifications/K7.8-task-completion-failure-and-evidence.md`, `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/completion.rs`, `crates/kernel-domain/src/task/completion_rules.rs`, `crates/kernel-domain/src/task/completion_validation.rs`, `crates/kernel-domain/src/task/failure.rs`, `crates/kernel-domain/src/task/failure_validation.rs`, `crates/kernel-domain/src/task/evidence.rs`, `crates/kernel-domain/src/task/evidence_set.rs`, `crates/kernel-domain/src/task/evidence_validation.rs`, `crates/kernel-domain/src/task/outcome_decision.rs`, `crates/kernel-domain/src/task/completion_tests.rs`, `crates/kernel-domain/src/task/failure_tests.rs`, `crates/kernel-domain/src/task/evidence_tests.rs`, `crates/kernel-domain/src/task/outcome_separation_tests.rs`, `crates/kernel-domain/src/task/outcome_test_support.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `crates/kernel-domain/src/errors.rs` | `TaskCompletionResult`, `TaskCompletion`, `TaskCompletionOutcome`, `TaskCompletionControl`, `TaskFailure`, `TaskFailureReason`, `TaskFailureReference`, `TaskRecoveryReference`, `TaskFailureOutcome`, `TaskFailureControl`, `TaskEvidence`, `TaskEvidenceType`, `TaskEvidenceMetadata`, `TaskEvidenceSet`, `TaskEvidenceValidation`, `TaskOutcomeDecision`, `DomainError::InvalidTaskCompletion`, `DomainError::InvalidTaskFailure`, `DomainError::InvalidTaskEvidence` | `task_completion_*`, `task_failure_*`, `task_evidence_*`, `task_outcome_*` | `IMPLEMENTED / VERIFIED` | `7ecb6b2` | `20` new tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |
| `K7-009` | Task integration and conformance remain explicit, additive, deterministic, immutable, identity-consistent, and architecture-preserving across the full task-domain composition flow without introducing a runtime facade. | `docs/specifications/K7.9-task-integration-and-conformance.md` | `crates/kernel-domain/src/task/integration_test_support.rs`, `crates/kernel-domain/src/task/integration_flow_support.rs`, `crates/kernel-domain/src/task/integration_completion_tests.rs`, `crates/kernel-domain/src/task/integration_failure_tests.rs`, `crates/kernel-domain/src/task/integration_dependency_tests.rs`, `crates/kernel-domain/src/task/integration_readiness_tests.rs`, `crates/kernel-domain/src/task/integration_identity_tests.rs`, `crates/kernel-domain/src/task/integration_sequence_tests.rs`, `crates/kernel-domain/src/task/integration_determinism_tests.rs`, `crates/kernel-domain/src/task/integration_separation_tests.rs`, `crates/kernel-domain/src/task/mod.rs`, `crates/kernel-domain/src/lib.rs`, `docs/API.md`, `docs/API-FREEZE.md`, `docs/TRACEABILITY.md`, `docs/VALIDATION.md`, `docs/IMPLEMENTATION-PLAN.md`, `docs/plans/K7-IMPLEMENTATION-PLAN.md`, `docs/backlog/K7-BACKLOG.md`, `CHANGELOG.md` | Conformance coverage over existing K7 public APIs; no new public subsystem exported | `integration_*` | `IMPLEMENTED / VERIFIED` | `5b7641e` | `28` new cross-module tests added. Native host verification now passes. CES requirement IDs: `UNRESOLVED — DO NOT FABRICATE` |

## K7 Boundaries Confirmed

- No task execution
- No scheduler
- No persistence
- No async runtime
- No event bus
- No implicit event publication
- No network
- No K1-K6 API redesign

## K7 Requirement Status Closure

- Implemented: `K7-001` through `K7-009`
- Native-tested requirements: `IMPLEMENTED / VERIFIED`
- Compile-gated requirements: `VERIFIED BY COMPILE GATES`
- Static-only requirements: `VERIFIED BY STATIC AUDIT`
- Deferred: downstream runtime and milestone work beyond K7 remain outside this traceability closure

## K7 Defect-Fix History

- `e7f8256`: corrected shared `TaskInstanceReference` fixture coupling
- `8bf4390`: corrected non-canonical `TaskFailurePolicyReference` fixtures
- `c2e8a36`: corrected non-canonical `AuthorizationDecisionId` fixtures

## K7 Closure Assertions

- Production behavior changed: `NO`
- Public API changed: `NO`
- Architecture changed: `NO`
- ADR required: `NO`

## Program Alignment

- Repository dependency direction remains `AI Engineering OS -> CHELA-X CES -> CHELA-X Kernel -> CHELA-X Runtime -> CHELA-X SDK -> CHELA-X Media`.
- K6 remains additive inside `kernel-domain` and does not redesign architecture or dependency direction.
