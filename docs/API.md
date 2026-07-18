# API

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-18

## Applies To
Public API review and consumption guidance for `kernel-domain`, including the frozen K6 workflow API.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose And Scope

This document records the current public K6 workflow API and the additive K7.1 through K7.6 task-domain APIs exposed from `crates/kernel-domain/src/lib.rs`. K6 remains frozen. K7-001 through K7-006 are implemented and not frozen.

## K6 Public API Surface

### Foundation Types

Types:

- `WorkflowRetryLimit`
- `WorkflowRetryPolicyReference`
- `WorkflowRecoveryReference`
- `WorkflowAuditEvidenceReference`
- `WorkflowLifecycleMapReference`
- `WorkflowStepReference`
- `WorkflowTerminalOutcomeReference`
- `WorkflowStepOutcomeReference`
- `WorkflowEngineFoundation`

Construction entry points:

- `WorkflowRetryLimit::new(value: u16) -> DomainResult<Self>`
- `WorkflowRetryPolicyReference::new(definition_version: StableVersion, retry_limit: WorkflowRetryLimit) -> Self`
- `WorkflowRecoveryReference::new(corrective_path: impl Into<String>, requires_revalidation: bool) -> DomainResult<Self>`
- `WorkflowAuditEvidenceReference::new(...) -> DomainResult<Self>`
- `WorkflowLifecycleMapReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `WorkflowStepReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `WorkflowTerminalOutcomeReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `WorkflowStepOutcomeReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `WorkflowEngineFoundation::new(...) -> DomainResult<Self>`

Principal accessors:

- `WorkflowRetryLimit::value(self) -> u16`
- `WorkflowRetryPolicyReference::definition_version(&self) -> &StableVersion`
- `WorkflowRetryPolicyReference::retry_limit(&self) -> WorkflowRetryLimit`
- `WorkflowRecoveryReference::corrective_path(&self) -> &str`
- `WorkflowRecoveryReference::requires_revalidation(&self) -> bool`
- `WorkflowAuditEvidenceReference::audit_evidence_id(&self) -> &AuditEvidenceId`
- `WorkflowLifecycleMapReference::as_str(&self) -> &str`
- `WorkflowStepReference::as_str(&self) -> &str`
- `WorkflowTerminalOutcomeReference::as_str(&self) -> &str`
- `WorkflowStepOutcomeReference::as_str(&self) -> &str`
- `WorkflowEngineFoundation::{workflow_id, ownership, definition_version, retry_policy, retry_limit, recovery_reference, audit_evidence}`

Deterministic behavior:

- Constructor-only validation
- Immutable post-construction state
- No lookup, scheduling, publishing, persistence, or execution

Validation boundaries:

- Retry limit must be positive
- Retry limit requires retry policy when carried by workflow foundation
- Audit evidence rejects incomplete consumed upstream references

Important non-goals:

- No workflow execution
- No runtime scheduler
- No persistence

### Definition Types

Types:

- `WorkflowDefinition`

Construction entry point:

- `WorkflowDefinition::new(...) -> DomainResult<Self>`

Principal accessors:

- `WorkflowDefinition::{workflow_id, namespace, definition_version, ownership, lifecycle_map, entry_steps, terminal_outcomes, policy_references, retry_policy, retry_limit, recovery_reference, audit_evidence}`

Deterministic behavior:

- Approved-definition model only
- Immutable and version-bound
- Preserves caller ordering for steps, terminal outcomes, policies, and evidence

Validation boundaries:

- Duplicate entry steps rejected
- Duplicate terminal outcomes rejected
- Duplicate policy references rejected
- Retry limit requires retry policy

Important non-goals:

- No approval engine
- No runtime schema lookup
- No task semantics

### Instance Types

Types:

- `WorkflowInstance`

Construction entry point:

- `WorkflowInstance::new(...) -> DomainResult<Self>`

Principal accessors:

- `WorkflowInstance::{workflow_id, workflow_definition, definition_version_snapshot, ownership_reference, current_workflow_state_snapshot, creation_evidence, retry_policy_snapshot, retry_limit_snapshot, recovery_reference, audit_evidence_references}`

Deterministic behavior:

- Captures one immutable execution instance snapshot
- Reuses canonical K2 workflow state
- Preserves supplied evidence ordering

Validation boundaries:

- Workflow definition is mandatory
- Definition version snapshot is mandatory
- Ownership reference is mandatory
- Workflow state snapshot is mandatory
- Retry limit requires retry policy
- Duplicate audit evidence rejected

Important non-goals:

- No persistence identifiers
- No external existence lookup
- No execution history mutation

### Transition-Control Types

Types:

- `WorkflowTransitionControlRequest`
- `WorkflowTransitionControl`
- `WorkflowTransitionDecision`
- `WorkflowTransitionOutcome`

Construction and evaluation entry points:

- `WorkflowTransitionControlRequest::new(...) -> DomainResult<Self>`
- `WorkflowTransitionControl::evaluate(request: &WorkflowTransitionControlRequest) -> WorkflowTransitionDecision`
- `validate_workflow_transition(request: &WorkflowTransitionRequest, guards: &WorkflowLifecycleGuards) -> WorkflowTransitionOutcome`

Principal accessors:

- `WorkflowTransitionControlRequest::{current_workflow_state_snapshot, requested_target_workflow_state, transition_reason_reference, transition_authority_reference, transition_evidence_references, failure_code, workflow_lifecycle_guards}`

Deterministic behavior:

- Delegates to the frozen K2 workflow lifecycle map
- Advances sequence only on allowed transitions
- Preserves no-op and rejected sequence semantics

Validation boundaries:

- Failure transitions require `WorkflowFailureCode`
- Non-failure transitions reject unrelated failure codes
- Duplicate transition evidence rejected
- Missing required K2 guard evidence remains rejected by K2

Important non-goals:

- No mutation of `WorkflowInstance`
- No scheduler
- No step or task runtime

### Step-Coordination Types

Types:

- `WorkflowStepSelection`
- `WorkflowStepExecutionPlan`
- `WorkflowStepCoordination`

Construction entry points:

- `WorkflowStepSelection::new(current_step: WorkflowStepReference, next_candidate_steps: Vec<WorkflowStepReference>) -> Self`
- `WorkflowStepExecutionPlan::new(...) -> DomainResult<Self>`
- `WorkflowStepCoordination::new(...) -> DomainResult<Self>`

Principal accessors:

- `WorkflowStepSelection::{current_step, next_candidate_steps}`
- `WorkflowStepExecutionPlan::{completed_step_references, blocked_step_references, skipped_step_references, terminal_step_references}`
- `WorkflowStepCoordination::{workflow_definition, workflow_instance, workflow_step_selection, workflow_step_execution_plan}`

Deterministic behavior:

- Declarative step coordination only
- Preserves caller ordering for all step-reference collections
- Immutable after construction

Validation boundaries:

- Duplicate completed, blocked, skipped, and terminal references rejected
- Current step cannot simultaneously be completed, blocked, or skipped

Important non-goals:

- No task creation
- No execution callbacks
- No scheduler

### Authorization-Integration Types

Types:

- `WorkflowOperationReference`
- `WorkflowAuthorizationContext`
- `WorkflowAuthorizationRequest`
- `WorkflowAuthorizationControl`
- `WorkflowAuthorizationDecision`

Construction and evaluation entry points:

- `WorkflowOperationReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `WorkflowAuthorizationContext::new(...) -> DomainResult<Self>`
- `WorkflowAuthorizationRequest::new(...) -> DomainResult<Self>`
- `WorkflowAuthorizationControl::evaluate(request: &WorkflowAuthorizationRequest) -> WorkflowAuthorizationDecision`
- `pub type WorkflowAuthorizationDecision = AuthorizationDecisionOutcome`

Principal accessors:

- `WorkflowOperationReference::as_str(&self) -> &str`
- `WorkflowAuthorizationContext::{authorization_request, authorization_decision, authorization_evidence_references}`
- `WorkflowAuthorizationRequest::{workflow_operation, workflow_definition, workflow_instance, current_workflow_state, requested_target_workflow_state, workflow_step_coordination, current_workflow_step, requested_next_workflow_step, workflow_authorization_context, transition_authority_reference, transition_evidence_references}`

Deterministic behavior:

- Consumes canonical K3 authorization facts by reference only
- Returns the supplied canonical authorization outcome
- Preserves caller ordering for transition evidence

Validation boundaries:

- Duplicate transition evidence rejected
- Workflow ownership must match the authorization scope where supplied
- No ownership-based permission inference

Important non-goals:

- No policy evaluation
- No role resolution
- No identity lookup

### Event-Integration Types

Types:

- `WorkflowEventTypeReference`
- `WorkflowEventContext`
- `WorkflowEventIntegrationRequest`
- `WorkflowEventIntegration`
- `WorkflowEventDecision`

Construction and evaluation entry points:

- `pub type WorkflowEventTypeReference = EventType`
- `WorkflowEventContext::new(...) -> DomainResult<Self>`
- `WorkflowEventIntegrationRequest::new(...) -> DomainResult<Self>`
- `WorkflowEventIntegration::evaluate(request: &WorkflowEventIntegrationRequest) -> DomainResult<WorkflowEventDecision>`
- `pub type WorkflowEventDecision = EventEnvelope<WorkflowEventContext>`

Principal accessors:

- `WorkflowEventContext::{workflow_definition, workflow_instance, workflow_state_snapshot, workflow_step_coordination, workflow_step_reference, workflow_transition_control_request, workflow_transition_decision, workflow_authorization_request, workflow_authorization_decision, workflow_operation, transition_reason_reference, transition_authority_reference, transition_evidence_references, workflow_audit_evidence_references, failure_code}`
- `WorkflowEventIntegrationRequest::{workflow_event_type, event_id, event_version, occurred_at, recorded_at, event_source, event_subject, event_classification, correlation_id, causation, workflow_event_context}`

Deterministic behavior:

- Constructs canonical K5 event envelopes only from explicit inputs
- Preserves caller-supplied correlation, causation, and evidence ordering
- Performs no publication or persistence

Validation boundaries:

- Workflow event type, event id, timestamp, source, and subject binding validated
- Transition and authorization categories must match supplied workflow facts
- Duplicate evidence rejected
- Failure event requires `WorkflowFailureCode`

Important non-goals:

- No event bus
- No outbox
- No system clock
- No UUID generation

## K7 Public API Surface

### Task Engine Foundation Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskDefinitionId`
- `TaskInstanceId`
- `TaskDependencyId`
- `TaskEvidenceId`
- `TaskDefinitionReference`
- `TaskInstanceReference`
- `TaskDependencyReference`
- `TaskEvidenceReference`
- `TaskWorkflowReference`
- `TaskStepReference`

Construction entry points:

- `TaskDefinitionId::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskInstanceId::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskDependencyId::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskEvidenceId::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskDefinitionReference::new(task_definition_id: TaskDefinitionId) -> Self`
- `TaskInstanceReference::new(task_instance_id: TaskInstanceId) -> Self`
- `TaskDependencyReference::new(task_dependency_id: TaskDependencyId) -> Self`
- `TaskEvidenceReference::new(task_evidence_id: TaskEvidenceId) -> Self`
- `TaskWorkflowReference::new(workflow_id: WorkflowId) -> Self`
- `TaskStepReference::new(workflow_step_reference: WorkflowStepReference) -> Self`

Principal accessors:

- `TaskDefinitionId::as_str(&self) -> &str`
- `TaskInstanceId::as_str(&self) -> &str`
- `TaskDependencyId::as_str(&self) -> &str`
- `TaskEvidenceId::as_str(&self) -> &str`
- `TaskDefinitionReference::task_definition_id(&self) -> &TaskDefinitionId`
- `TaskInstanceReference::task_instance_id(&self) -> &TaskInstanceId`
- `TaskDependencyReference::task_dependency_id(&self) -> &TaskDependencyId`
- `TaskEvidenceReference::task_evidence_id(&self) -> &TaskEvidenceId`
- `TaskWorkflowReference::workflow_id(&self) -> &WorkflowId`
- `TaskStepReference::workflow_step_reference(&self) -> &WorkflowStepReference`

Deterministic behavior:

- Constructor-only validation for string-backed task vocabulary
- Immutable value semantics after construction
- Deterministic equality, ordering, hashing, and debug output through K1 conventions
- No hidden clock, randomness, persistence, scheduling, or event publication

Validation boundaries:

- Canonical task identities reject empty values and unsafe identifier characters
- References preserve the exact canonical identifier type they wrap
- Workflow and step task references reuse the upstream `WorkflowId` and `WorkflowStepReference` types directly
- No public `TaskId`
- No generic public `TaskReference`
- No cross-type conversions between definition and instance identities

Important non-goals:

- No task definition model
- No task instance lifecycle
- No ownership rules
- No readiness or transition evaluation
- No dependency validation
- No completion or failure semantics beyond vocabulary
- No scheduler, executor, persistence, async runtime, or network integration

### Task Definition Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskDefinition`
- `TaskDefinitionVersion`
- `TaskDefinitionName`
- `TaskDescription`
- `TaskKind`
- `TaskInputContract`
- `TaskOutputContract`
- `TaskRequirement`
- `TaskCapabilityRequirement`
- `TaskEvidenceRequirement`
- `TaskCompletionRequirement`
- `TaskFailurePolicyReference`

Construction entry points:

- `TaskDefinition::new(...) -> DomainResult<Self>`
- `TaskDefinitionVersion::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskDefinitionName::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskDescription::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskKind::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskInputContract::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskOutputContract::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskRequirement::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskCapabilityRequirement::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskEvidenceRequirement::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskCompletionRequirement::new(value: impl Into<String>) -> DomainResult<Self>`

Principal accessors:

- `TaskDefinition::{task_definition_id, task_definition_version, task_definition_name, task_description, task_kind, task_input_contracts, task_output_contracts, task_requirements, task_capability_requirements, task_evidence_requirements, task_completion_requirements, task_failure_policy_reference, task_workflow_reference, task_step_reference}`
- `TaskDefinitionVersion::as_str(&self) -> &str`
- `TaskDefinitionName::as_str(&self) -> &str`
- `TaskDescription::as_str(&self) -> &str`
- `TaskKind::as_str(&self) -> &str`
- `TaskInputContract::as_str(&self) -> &str`
- `TaskOutputContract::as_str(&self) -> &str`
- `TaskRequirement::as_str(&self) -> &str`
- `TaskCapabilityRequirement::as_str(&self) -> &str`
- `TaskEvidenceRequirement::as_str(&self) -> &str`
- `TaskCompletionRequirement::as_str(&self) -> &str`

Deterministic behavior:

- Task definitions are immutable after construction
- Caller-supplied ordering is preserved for inputs, outputs, and all requirement collections
- Explicit workflow and step bindings remain optional and data-only
- No id generation, clock access, persistence, execution, or publication occurs

Validation boundaries:

- Task definition requires at least one input contract
- Task definition requires at least one completion requirement
- Duplicate task input, output, general requirement, capability requirement, evidence requirement, and completion requirement declarations are rejected
- Task step binding requires workflow binding
- Version, name, description, kind, and contract values reuse existing validated K1 primitives

Important non-goals:

- No task instance creation
- No assignment state
- No readiness evaluation
- No lifecycle state
- No dependency execution
- No scheduler, executor, persistence, or network integration

### Task Instance Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskInstance`
- `TaskDefinitionSnapshotReference`
- `TaskCreationContext`
- `TaskInputBinding`
- `TaskOutputBinding`
- `TaskWorkflowBinding`
- `TaskStepBinding`
- `TaskState`

Construction entry points:

- `TaskInstance::new(...) -> DomainResult<Self>`
- `TaskDefinitionSnapshotReference::new(task_definition_reference: TaskDefinitionReference, task_definition_version: TaskDefinitionVersion) -> Self`
- `TaskCreationContext::new(task_input_bindings: Vec<TaskInputBinding>, task_creation_authority: Option<TransitionAuthorityReference>) -> DomainResult<Self>`
- `TaskInputBinding::new(task_input_contract: TaskInputContract) -> Self`
- `TaskOutputBinding::new(task_output_contract: TaskOutputContract) -> Self`
- `TaskWorkflowBinding::from_workflow_definition(workflow_definition: WorkflowDefinition) -> Self`
- `TaskWorkflowBinding::from_workflow_instance(workflow_instance: WorkflowInstance) -> Self`
- `TaskStepBinding::new(task_step_reference: TaskStepReference) -> Self`

Principal accessors:

- `TaskInstance::{task_instance_id, task_definition, task_definition_snapshot_reference, task_creation_context, task_output_bindings, task_workflow_binding, task_step_binding, task_state}`
- `TaskDefinitionSnapshotReference::{task_definition_reference, task_definition_version}`
- `TaskCreationContext::{task_input_bindings, task_creation_authority}`
- `TaskInputBinding::task_input_contract(&self) -> &TaskInputContract`
- `TaskOutputBinding::task_output_contract(&self) -> &TaskOutputContract`
- `TaskWorkflowBinding::{workflow_definition, workflow_instance, workflow_id}`
- `TaskStepBinding::task_step_reference(&self) -> &TaskStepReference`
- `TaskState::as_str(self) -> &'static str`

Deterministic behavior:

- Task-instance creation is explicit and immutable
- Definition identity and version are snapshot-bound at construction
- Caller-supplied input and output ordering is preserved
- Initial lifecycle representation is explicit and restricted to `Pending`
- No lookup, execution, persistence, event publication, scheduler interaction, clock access, or random generation occurs

Validation boundaries:

- Task instance requires a valid `TaskDefinition`
- Initial state must be `Pending`
- Step binding requires workflow binding
- Missing required task input bindings are rejected
- Input bindings not declared by the task definition are rejected
- Output bindings not declared by the task definition are rejected
- Duplicate input bindings are rejected
- Duplicate output bindings are rejected
- Workflow and step bindings must match task-definition workflow and step bindings where the definition declares them

Important non-goals:

- No transition engine
- No readiness evaluation
- No assignment or ownership behavior
- No dependency execution
- No completion or failure outcomes
- No executor, scheduler, persistence, or network integration

### Task Ownership And Assignment Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskOwner`
- `TaskOwnership`
- `TaskOwnershipAuthority`
- `TaskOwnershipScope`
- `TaskAssignee`
- `TaskAssignment`
- `TaskAssignmentAuthority`
- `TaskAssignmentStatus`
- `TaskAssignmentReason`
- `TaskAssignmentRejectionReason`
- `TaskAssignmentRequest`
- `TaskAssignmentChange`
- `TaskAssignmentRejection`
- `TaskAssignmentNoOp`
- `TaskAssignmentDecision`
- `TaskAssignmentControl`

Construction and evaluation entry points:

- `TaskOwner::{from_identity, from_ownership_subject}`
- `TaskOwnership::new(task_instance_reference: TaskInstanceReference, task_owner: TaskOwner, task_ownership_scope: TaskOwnershipScope, task_ownership_authority: TaskOwnershipAuthority) -> Self`
- `TaskAssignee::{from_identity, from_ownership_subject}`
- `TaskAssignment::new(...) -> DomainResult<Self>`
- `TaskAssignmentReason::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskAssignmentRequest::new(...) -> Self`
- `TaskAssignmentControl::evaluate(request: &TaskAssignmentRequest) -> TaskAssignmentDecision`

Principal accessors:

- `TaskOwner::{identity_reference, ownership_subject}`
- `TaskOwnership::{task_instance_reference, task_owner, task_ownership_scope, task_ownership_authority}`
- `TaskAssignee::{identity_reference, ownership_subject}`
- `TaskAssignment::{task_instance_reference, task_assignee, task_assignment_status, task_assignment_authority, task_assignment_reason}`
- `TaskAssignmentReason::as_str(&self) -> &str`
- `TaskAssignmentRequest::{current_assignment, requested_assignee, task_assignment_authority, task_assignment_reason, authorization_outcome}`
- `TaskAssignmentChange::{previous_assignment, current_assignment}`
- `TaskAssignmentRejection::{current_assignment, reason}`
- `TaskAssignmentNoOp::current_assignment(&self) -> &TaskAssignment`

Deterministic behavior:

- Ownership and assignment are modeled as separate immutable snapshots
- Assignment decisions return explicit `Updated`, `Rejected`, or `NoOp` outcomes
- Reassignment preserves previous assignment state in the returned change record
- Assignment consumes explicit K3 authorization outcomes and explicit K4 lifecycle facts only
- No lifecycle transition, readiness evaluation, execution, persistence, event publication, scheduler interaction, clock access, or random generation occurs

Validation boundaries:

- Ownership requires explicit owner, scope, authority, and task-instance reference by type
- `Unassigned` assignment snapshots reject retained assignee, authority, or reason
- `Assigned` and `Accepted` snapshots require assignee and authority
- `Rejected`, `Released`, and `Revoked` snapshots require cleared assignee, authority, and reason
- Assignment without required authority is rejected deterministically
- Unassignment without required reason is rejected deterministically
- Authorization-denied outcomes are rejected without consulting external policy services
- Human assignees in `Retirement` or `Archive` are rejected when those explicit upstream lifecycle facts are supplied
- Agent assignees in `Paused`, `Suspended`, `Recovering`, `Retired`, or `Deleted` are rejected when those explicit upstream lifecycle facts are supplied

Important non-goals:

- No ownership transfer workflow
- No readiness evaluation
- No lifecycle transition execution
- No dependency resolution
- No task execution or worker dispatch
- No policy redefinition, persistence, event publication, scheduler, executor, or network integration

### Task Priority And Readiness Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskPriority`
- `TaskPriorityClass`
- `TaskPriorityValue`
- `TaskReadiness`
- `TaskReadinessRequirement`
- `TaskReadinessEvidence`
- `TaskReadinessBlocker`
- `TaskReadinessRejectionReason`
- `TaskReadinessInput`
- `TaskReadinessReady`
- `TaskReadinessBlocked`
- `TaskReadinessRejection`
- `TaskReadinessDecision`
- `TaskReadinessControl`

Construction and evaluation entry points:

- `TaskPriorityClass::new(value: &str) -> DomainResult<Self>`
- `TaskPriorityValue::new(value: u8) -> DomainResult<Self>`
- `TaskPriority::new(task_instance_reference: TaskInstanceReference, task_priority_class: TaskPriorityClass, task_priority_value: TaskPriorityValue) -> Self`
- `TaskReadinessInput::new(...) -> Self`
- `TaskReadinessControl::evaluate(input: &TaskReadinessInput) -> TaskReadinessDecision`

Principal accessors:

- `TaskPriority::{task_instance_reference, task_priority_class, task_priority_value}`
- `TaskPriorityClass::as_str(self) -> &'static str`
- `TaskPriorityValue::value(self) -> u8`
- `TaskReadinessInput::{task_instance_reference, task_state, task_priority, task_ownership, task_assignment, task_readiness_requirements, task_readiness_evidence, authorization_outcome}`
- `TaskReadinessReady::{task_instance_reference, task_readiness, validated_evidence}`
- `TaskReadinessBlocked::{task_instance_reference, task_readiness, blockers}`
- `TaskReadinessRejection::{task_instance_reference, reason}`

Deterministic behavior:

- Priority is explicit immutable governance metadata bound to `TaskInstanceReference`
- Priority comparison uses validated `TaskPriorityValue` ordering only
- Equal priority remains valid and stable
- Readiness consumes explicit lifecycle, ownership, assignment, authorization, and evidence facts only
- Readiness returns only derived `Ready`, `Blocked`, or structural `Rejected` outcomes
- Same input produces the same readiness decision
- No lifecycle mutation, assignment mutation, scheduling, execution, resource reservation, wall-clock lookup, or randomness occurs

Validation boundaries:

- Only canonical priority class `Explicit` is accepted
- Priority value must be greater than zero
- Contradictory readiness requirements are rejected deterministically
- Terminal task states are blocked, not promoted to lifecycle `Ready`
- Missing required owner, assignment, required input, dependency completion, authorization allowance, or evidence prerequisites yield stable readiness blockers
- Accepted assignment remains distinct from readiness; assignment status is read but never changed
- High priority remains distinct from readiness; supplied priority is preserved but never interpreted as scheduler authorization

Important non-goals:

- No scheduler queue
- No execution dispatch
- No dependency graph traversal or mutation
- No authorization service lookup
- No worker, executor, or capacity semantics
- No lifecycle transition engine

### Task Lifecycle And State Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `NOT FROZEN`

Types:

- `TaskStateSnapshot`
- `TaskFailureCode`
- `TaskFailureCategory`
- `TaskLifecycleGuards`
- `TaskTransitionRequest`
- `TaskTransitionRejectionReason`
- `TaskAllowedTransition`
- `TaskRejectedTransition`
- `TaskNoOpTransition`
- `TaskTransitionDecision`
- `TaskTransitionControl`

Construction and evaluation entry points:

- `TaskStateSnapshot::new(task_instance_reference: TaskInstanceReference, task_state: TaskState, state_sequence: StateSequence) -> Self`
- `TaskFailureCode::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskFailureCategory::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskLifecycleGuards::new(...) -> Self`
- `TaskTransitionRequest::new(...) -> DomainResult<Self>`
- `TaskTransitionControl::evaluate(request: &TaskTransitionRequest) -> TaskTransitionDecision`

Principal accessors:

- `TaskStateSnapshot::{task_instance_reference, task_state, state_sequence}`
- `TaskFailureCode::as_str(&self) -> &str`
- `TaskFailureCategory::as_str(&self) -> &str`
- `TaskLifecycleGuards::{expected_current_sequence, assignment_required, authorization_allowed, dependencies_satisfied, completion_conditions_met, required_outputs_present, required_completion_evidence_present, required_failure_evidence_present, failure_code, failure_category}`
- `TaskTransitionRequest::{current_task_state_snapshot, requested_target_task_state, transition_reason_reference, transition_authority_reference, transition_evidence_references, task_readiness_decision, task_assignment, task_lifecycle_guards}`
- `TaskAllowedTransition::{previous_task_state_snapshot, current_task_state_snapshot, transition_reason_reference, transition_authority_reference, transition_evidence_references}`
- `TaskRejectedTransition::{current_task_state_snapshot, requested_target_task_state, reason}`
- `TaskNoOpTransition::current_task_state_snapshot(&self) -> &TaskStateSnapshot`

Allowed transitions:

- `Pending -> InProgress`
- `Pending -> Cancelled`
- `InProgress -> Completed`
- `InProgress -> Failed`
- `InProgress -> Cancelled`
- `Completed -> Archived`
- `Failed -> Archived`
- `Cancelled -> Archived`

Rejected transitions:

- Illegal edges return `TaskTransitionRejectionReason::IllegalTransition`
- `Archived -> *` returns `TaskTransitionRejectionReason::TerminalState`
- Operational transitions from `Completed`, `Failed`, or `Cancelled` return `TaskTransitionRejectionReason::TerminalState`
- Sequence mismatch returns `TaskTransitionRejectionReason::SequenceMismatch`
- Missing cancellation authority or archival authority returns `TaskTransitionRejectionReason::MissingAuthority`
- Missing cancellation reason returns `TaskTransitionRejectionReason::MissingReason`
- Missing completion or failure evidence returns `TaskTransitionRejectionReason::MissingEvidence`
- Start without supplied `Ready` readiness returns `TaskTransitionRejectionReason::ReadinessNotSatisfied`

Deterministic behavior:

- Lifecycle snapshot preserves explicit `TaskInstanceReference`, `TaskState`, and `StateSequence`
- Same-state requests return `NoOp`
- Allowed transitions advance sequence exactly once
- Rejected transitions preserve the supplied snapshot and requested target state
- Transition control consumes explicit readiness, assignment, and lifecycle facts only
- No hidden lookup, scheduling, execution, persistence, publication, clock access, or randomness occurs

Validation boundaries:

- Duplicate transition evidence references are rejected at request construction
- Start requires explicit supplied readiness `Ready`
- Start requires accepted assignment only when assignment is explicitly required by the supplied lifecycle guards
- Start requires explicit authorization and dependency satisfaction facts from the supplied lifecycle guards
- Completion requires explicit completion conditions, outputs, and evidence facts
- Failure requires explicit stable failure code, deterministic failure category, and failure evidence fact
- Cancellation requires explicit authority and reason
- Archival requires explicit authority

Important non-goals:

- No dependency coordination or graph traversal
- No readiness evaluation inside transition control
- No assignment mutation or ownership mutation
- No completion record storage
- No failure record storage
- No runtime orchestration or execution start

### Failure-And-Recovery Types

Types:

- `WorkflowFailureContext`
- `WorkflowFailureRecord`
- `WorkflowRecoveryRequest`
- `WorkflowRecoveryControl`
- `WorkflowRecoveryDecision`

Construction and evaluation entry points:

- `WorkflowFailureContext::new(...) -> DomainResult<Self>`
- `WorkflowFailureRecord::new(...) -> DomainResult<Self>`
- `WorkflowRecoveryRequest::new(workflow_failure_record: WorkflowFailureRecord, recovery_revalidated: bool) -> DomainResult<Self>`
- `WorkflowRecoveryControl::evaluate(request: &WorkflowRecoveryRequest) -> WorkflowRecoveryDecision`

Principal accessors:

- `WorkflowFailureContext::{workflow_id, workflow_instance, current_workflow_state, current_workflow_step, failure_code, transition_reason_reference, transition_authority_reference, transition_evidence_references, workflow_audit_evidence_references, correlation_id, causation}`
- `WorkflowFailureRecord::{workflow_failure_context, retry_policy_reference, retry_limit, current_retry_attempt, recovery_reference, recovery_target_state, failure_sequence}`
- `WorkflowRecoveryRequest::{workflow_failure_record, recovery_revalidated}`
- `WorkflowRecoveryDecision::workflow_failure_record(&self) -> &WorkflowFailureRecord`

Deterministic behavior:

- Recovery decisions are pure and explicit
- Retry allowance depends only on explicit retry counters and limits
- Terminal workflows short-circuit deterministically

Validation boundaries:

- Failure code is mandatory by type
- Retry limit requires retry policy
- Retry attempt must not exceed retry limit
- Recovery target requires recovery reference and vice versa
- Workflow identity and instance snapshots must remain internally consistent
- Duplicate transition evidence and workflow audit evidence rejected

Important non-goals:

- No retry queue
- No scheduler
- No backoff calculation
- No workflow transition execution

### Task-And-Workflow Domain Errors

Public variants:

- `DomainError::InvalidTaskDefinition(&'static str)`
- `DomainError::InvalidTaskInstance(&'static str)`
- `DomainError::InvalidTaskLifecycle(&'static str)`
- `DomainError::InvalidTaskPriority(&'static str)`
- `DomainError::InvalidTaskReadiness(&'static str)`
- `DomainError::InvalidTaskOwnership(&'static str)`
- `DomainError::InvalidTaskAssignment(&'static str)`
- `DomainError::InvalidWorkflowReference(&'static str)`
- `DomainError::InvalidWorkflowDefinition(&'static str)`
- `DomainError::InvalidWorkflowInstance(&'static str)`
- `DomainError::InvalidWorkflowTransitionControl(&'static str)`
- `DomainError::InvalidWorkflowStepCoordination(&'static str)`
- `DomainError::InvalidWorkflowAuthorizationIntegration(&'static str)`
- `DomainError::InvalidWorkflowEventIntegration(&'static str)`
- `DomainError::InvalidWorkflowFailureRecovery(&'static str)`

These variants report constructor or binding failures only. They do not imply runtime execution, external lookup, or background workflow behavior.

## K6 Non-Goals

- No workflow execution engine
- No scheduler
- No executor
- No persistence
- No event bus
- No network transport
- No async runtime
- No external policy or identity lookup
