# API

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-19

## Applies To
Public API review and consumption guidance for `kernel-domain` and `kernel-gateway`, including the frozen K10 API Gateway surface.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose And Scope

This document records the current public K6 workflow API, the frozen K7 task-domain API, the additive frozen K8 execution-domain API exposed from `crates/kernel-domain/src/lib.rs`, and the frozen K10 gateway API exposed from `crates/kernel-gateway/src/lib.rs`.

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

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

### Task Dependency Coordination Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

Types:

- `TaskDependencyGraphReference`
- `TaskDependencySource`
- `TaskDependencyTarget`
- `TaskDependencyType`
- `TaskDependencyRequirement`
- `TaskDependencyStatus`
- `TaskDependency`
- `TaskDependencyFact`
- `TaskDependencySet`
- `TaskDependencyValidationRequest`
- `TaskDependencyCoordinationRequest`
- `TaskDependencyBlocker`
- `TaskDependencyUnresolvedReason`
- `TaskDependencyRejectionReason`
- `TaskDependencyValidationAccepted`
- `TaskDependencyValidationNoOp`
- `TaskDependencyValidationRejected`
- `TaskDependencyValidation`
- `TaskDependencyDecision`
- `TaskDependencyCoordinationDecision`
- `TaskDependencyControl`

Construction and evaluation entry points:

- `TaskDependencyGraphReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskDependencySource::new(task_instance_reference: TaskInstanceReference) -> Self`
- `TaskDependencyTarget::new(task_instance_reference: TaskInstanceReference) -> Self`
- `TaskDependency::new(...) -> DomainResult<Self>`
- `TaskDependencyFact::new(...) -> DomainResult<Self>`
- `TaskDependencySet::new(task_dependency_graph_reference: TaskDependencyGraphReference, task_dependencies: Vec<TaskDependency>) -> Self`
- `TaskDependencyValidationRequest::new(current_task_dependency_set: TaskDependencySet, requested_task_dependency: TaskDependency) -> Self`
- `TaskDependencyCoordinationRequest::new(task_dependency_set: TaskDependencySet, task_dependency_facts: Vec<TaskDependencyFact>) -> DomainResult<Self>`
- `TaskDependencyControl::validate(request: &TaskDependencyValidationRequest) -> TaskDependencyValidation`
- `TaskDependencyControl::evaluate(request: &TaskDependencyCoordinationRequest) -> TaskDependencyCoordinationDecision`

Principal accessors:

- `TaskDependencyGraphReference::as_str(&self) -> &str`
- `TaskDependencySource::task_instance_reference(&self) -> &TaskInstanceReference`
- `TaskDependencyTarget::task_instance_reference(&self) -> &TaskInstanceReference`
- `TaskDependency::{task_dependency_reference, task_dependency_source, task_dependency_target, task_dependency_type, task_dependency_requirement}`
- `TaskDependencyFact::{task_state_snapshot, task_evidence_references, task_output_contracts}`
- `TaskDependencySet::{task_dependency_graph_reference, task_dependencies}`
- `TaskDependencyValidationRequest::{current_task_dependency_set, requested_task_dependency}`
- `TaskDependencyCoordinationRequest::{task_dependency_set, task_dependency_facts}`
- `TaskDependencyValidationAccepted::task_dependency_set(&self) -> &TaskDependencySet`
- `TaskDependencyValidationNoOp::task_dependency_set(&self) -> &TaskDependencySet`
- `TaskDependencyValidationRejected::{task_dependency_set, requested_task_dependency, reason}`
- `TaskDependencyDecision::{task_dependency, task_dependency_status, task_dependency_blocker, task_dependency_unresolved_reason}`
- `TaskDependencyCoordinationDecision::{task_dependency_graph_reference, task_dependency_status, task_dependency_decisions, task_dependency_rejection_reason}`

Dependency types:

- `Completion`
- `Success`
- `Evidence`
- `Output`

Satisfaction and coordination behavior:

- Validation request returns `Accepted`, `Rejected`, or `NoOp`
- Coordination evaluation returns aggregate `Satisfied`, `Unsatisfied`, `Unresolved`, or `Rejected`
- Same semantic edge plus same `TaskDependencyId` returns `NoOp`
- Same semantic edge plus different `TaskDependencyId` returns `Rejected`
- Same `TaskDependencyId` reused for different semantic edge also returns `Rejected`
- Aggregate dependency status preserves dependency ordering from the supplied set

Cycle validation:

- Detects direct cycle through rejected self-edge construction
- Detects two-node and longer detectable cycles in the explicit supplied dependency set
- Accepts acyclic chains and disconnected acyclic groups
- Performs in-memory validation only over the supplied immutable set

Deterministic behavior:

- Dependency direction remains explicit through `TaskDependencySource` and `TaskDependencyTarget`
- Dependency validation and evaluation are side-effect free
- Readiness contribution is returned as explicit dependency status only
- No lifecycle mutation, readiness mutation, assignment mutation, execution start, event publication, persistence, clock lookup, or randomness occurs

Validation boundaries:

- Self-dependency is rejected at construction
- Dependency type and requirement pairing must match
- Duplicate evidence and output facts are rejected at fact construction
- Duplicate predecessor facts are rejected at coordination-request construction
- Missing predecessor fact produces `Unresolved`
- Completion, success, evidence, and output requirements are evaluated only from explicit supplied facts
- Cross-workflow orchestration remains deferred and is not inferred

Important non-goals:

- No generic graph framework
- No scheduler or execution ordering runtime
- No worker dispatch or queue management
- No repository or database graph lookup
- No automatic downstream mutation or propagation
- No readiness evaluation invocation from dependency control

### Task Completion, Failure, And Evidence Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

Types:

- `TaskOutputReference`
- `TaskOutput`
- `TaskEvidenceType`
- `TaskEvidenceMetadata`
- `TaskEvidence`
- `TaskEvidenceSet`
- `TaskEvidenceValidationRequest`
- `TaskEvidenceRejectionReason`
- `TaskEvidenceRejected`
- `TaskEvidenceValidation`
- `TaskEvidenceControl`
- `TaskCompletionResult`
- `TaskCompletion`
- `TaskCompletionValidationRequest`
- `TaskCompletionRejectionReason`
- `TaskCompletionRejected`
- `TaskCompletionOutcome`
- `TaskCompletionControl`
- `TaskFailureReason`
- `TaskFailureReference`
- `TaskRecoveryReference`
- `TaskFailure`
- `TaskFailureValidationRequest`
- `TaskFailureRejectionReason`
- `TaskFailureRejected`
- `TaskFailureOutcome`
- `TaskFailureControl`
- `TaskOutcomeDecision`
- `TaskOutcomeRejectionReason`

Construction and evaluation entry points:

- `TaskOutputReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskOutput::new(task_output_reference: TaskOutputReference, task_output_binding: TaskOutputBinding) -> Self`
- `TaskEvidenceType::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskEvidenceMetadata::new(task_evidence_requirement: Option<TaskEvidenceRequirement>, transition_evidence_reference: Option<TransitionEvidenceReference>) -> Self`
- `TaskEvidence::new(...) -> Self`
- `TaskEvidenceSet::new(task_instance_reference: TaskInstanceReference, task_evidences: Vec<TaskEvidence>) -> DomainResult<Self>`
- `TaskEvidenceValidationRequest::new(task_instance: TaskInstance, task_evidence_set: TaskEvidenceSet) -> Self`
- `TaskEvidenceControl::validate(request: &TaskEvidenceValidationRequest) -> TaskEvidenceValidation`
- `TaskCompletionResult::new(...) -> DomainResult<Self>`
- `TaskCompletionValidationRequest::new(...) -> Self`
- `TaskCompletionControl::evaluate(request: &TaskCompletionValidationRequest) -> TaskCompletionOutcome`
- `TaskFailureReason::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskFailureReference::new(value: impl Into<String>) -> DomainResult<Self>`
- `TaskRecoveryReference::new(corrective_path: impl Into<String>, requires_revalidation: bool) -> DomainResult<Self>`
- `TaskFailure::new(...) -> Self`
- `TaskFailureValidationRequest::new(...) -> Self`
- `TaskFailureControl::evaluate(request: &TaskFailureValidationRequest) -> TaskFailureOutcome`

Principal accessors:

- `TaskOutputReference::as_str(&self) -> &str`
- `TaskOutput::{task_output_reference, task_output_binding}`
- `TaskEvidenceType::as_str(&self) -> &str`
- `TaskEvidenceMetadata::{task_evidence_requirement, transition_evidence_reference}`
- `TaskEvidence::{task_evidence_reference, subject_task_instance_reference, task_evidence_type, producer_authority_reference, task_evidence_metadata}`
- `TaskEvidenceSet::{task_instance_reference, task_evidences}`
- `TaskEvidenceValidationRequest::{task_instance, task_evidence_set}`
- `TaskEvidenceRejected::{task_evidence_set, reason}`
- `TaskCompletionResult::{task_instance_reference, task_definition_snapshot_reference, task_completion_requirements, task_outputs, task_evidence_set, completion_authority_reference, completion_reason_reference}`
- `TaskCompletion::task_completion_result(&self) -> &TaskCompletionResult`
- `TaskCompletionValidationRequest::{task_instance, task_state_snapshot, task_completion_result, task_recovery_reference}`
- `TaskCompletionRejected::{task_completion_result, reason}`
- `TaskFailureReason::as_str(&self) -> &str`
- `TaskFailureReference::as_str(&self) -> &str`
- `TaskRecoveryReference::{corrective_path, requires_revalidation}`
- `TaskFailure::{task_instance_reference, task_failure_reference, task_failure_code, task_failure_category, task_failure_reason, task_failure_evidence_set, task_failure_authority_reference, task_failure_policy_reference}`
- `TaskFailureValidationRequest::{task_instance, task_state_snapshot, task_failure, task_completion}`
- `TaskFailureRejected::{task_failure, reason}`

Completion requirement and output validation:

- Completion remains explicit and deterministic; it is not inferred from `TaskState::Completed`
- Completion validation requires exact `TaskInstanceReference` and `TaskDefinitionSnapshotReference` match
- Every declared `TaskCompletionRequirement` from the supplied task definition must be present
- Every declared `TaskOutputContract` must have exactly one explicit `TaskOutput` binding
- Undeclared output contracts are rejected
- Duplicate output bindings by contract are rejected
- Output ordering is preserved exactly as supplied

Evidence model and validation:

- Evidence remains identity-bearing and infrastructure-neutral through `TaskEvidenceReference`, `TaskEvidenceType`, and typed metadata only

## K8 Public API Surface

### Execution Domain Types

API status:

- `IMPLEMENTED — REVIEW PASSED`
- `FROZEN FOR NEXT-MILESTONE CONSUMPTION`

Types:

- `ExecutionSessionId`
- `ExecutionRequest`
- `ExecutionContext`
- `ExecutionSession`
- `ExecutionOutcome`
- `ExecutionTermination`
- `ExecutionEvidenceBinding`
- `ExecutionRetryEligibilityDecision`
- `ExecutionRetryIneligibilityReason`
- `ExecutionAuditReference`
- `ExecutionValidation`

Construction and evaluation entry points:

- `ExecutionSessionId::new(value: impl Into<String>) -> DomainResult<Self>`
- `ExecutionEvidenceBinding::new(...) -> DomainResult<Self>`
- `ExecutionAuditReference::new(...) -> DomainResult<Self>`
- `ExecutionRequest::new(...) -> DomainResult<Self>`
- `ExecutionContext::new(...) -> DomainResult<Self>`
- `ExecutionSession::new(...) -> DomainResult<Self>`
- `ExecutionOutcome::{succeeded, failed, terminated}`
- `ExecutionRetryEligibilityDecision::evaluate(execution_outcome: &ExecutionOutcome, recovery_eligibility: Option<&RecoveryEligibility>) -> Self`
- `ExecutionValidation::{validate_request, validate_context, validate_session, validate_failed_snapshot, validate_reference_only_audit}`

Principal accessors:

- `ExecutionSessionId::as_str(&self) -> &str`
- `ExecutionEvidenceBinding::{execution_session_id, task_instance_reference, task_evidence_references, task_output_references, transition_evidence_references}`
- `ExecutionAuditReference::{execution_session_id, correlation_id, audit_evidence_ids}`
- `ExecutionRequest::{execution_session_id, task_instance_reference, task_state_snapshot, task_readiness_decision, authorization_decision_reference, requested_at}`
- `ExecutionContext::{execution_session_id, task_instance_reference, runtime_state_snapshot, delegation_reference, task_workflow_reference, task_step_reference, task_input_bindings}`
- `ExecutionSession::{execution_request, execution_context, execution_evidence_binding, execution_audit_reference, started_at}`
- `ExecutionOutcome::execution_session(&self) -> &ExecutionSession`
- `ExecutionTermination::as_str(&self) -> &'static str`

Deterministic behavior:

- K8 execution contracts remain pure, immutable, explicit, and side-effect free
- Execution consumes frozen K1-K7 facts only and never performs runtime lookup
- Outcomes are explicit and mutually exclusive
- Retry eligibility is derived from explicit supplied failure and recovery facts only
- No scheduler, worker, queue, process execution, network, filesystem, database, or event publication occurs

Validation boundaries:

- `ExecutionRequest` requires matching task identity across instance, snapshot, and readiness
- `ExecutionRequest` requires supplied readiness `Ready`, supplied `InProgress` task snapshot, and allowed authorization decision
- `ExecutionContext` rejects offline or retired runtime snapshots, invalid lease facts, and duplicate input bindings
- `ExecutionSession` preserves identity continuity across request, context, evidence, and audit references
- `ExecutionEvidenceBinding` rejects empty evidence or output sets and duplicate references
- `ExecutionOutcome` rejects task mismatch between execution session and accepted completion or failure facts
- `ExecutionAuditReference` is reference-only and rejects empty audit evidence

Important non-goals:

- No scheduler
- No worker dispatch
- No queue semantics
- No process spawning
- No network or filesystem access
- No database or memory persistence
- No automatic retry execution
- No automatic timeout execution
- No task lifecycle mutation
- Evidence must bind an explicit subject `TaskInstanceReference`
- Duplicate evidence identity is rejected where detectable
- Evidence metadata may carry only declared `TaskEvidenceRequirement` and `TransitionEvidenceReference`
- Undeclared evidence requirements are rejected
- Evidence ordering is preserved exactly as supplied

Failure classification and policy boundary:

- Failure uses canonical `TaskFailureCode` and `TaskFailureCategory`
- `TaskFailureReason` remains supplementary human-readable context and is not canonical identity
- `TaskFailureReference` remains a stable traceable reference and does not replace failure code
- `TaskFailurePolicyReference` is consumed by reference only and must match the supplied task definition when present
- No retry scheduling, recovery execution, backoff, persistence, stack-trace storage, or runtime execution behavior exists

Outcome decisions and separation:

- Completion and failure remain distinct from structural rejection
- `TaskCompletionOutcome` and `TaskFailureOutcome` return accepted fact or deterministic rejection
- `TaskOutcomeDecision` is mutually exclusive by type: `Completed`, `Failed`, or `Rejected`
- Completion and failure validation do not mutate `TaskState`
- Completion and failure validation do not call dependency control, readiness control, scheduler, worker dispatch, or persistence

Important non-goals:

- No execution engine
- No retry runtime
- No object or evidence storage
- No file, network, or database access
- No lifecycle mutation inside completion or failure validation

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
- `DomainError::InvalidTaskDependency(&'static str)`
- `DomainError::InvalidTaskCompletion(&'static str)`
- `DomainError::InvalidTaskFailure(&'static str)`
- `DomainError::InvalidTaskEvidence(&'static str)`
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

## K7 Integration And Conformance Review

Review status:

- `K7-009 IMPLEMENTED — REVIEW PASSED`
- `K7 IMPLEMENTATION COMPLETE`
- `K7 API FROZEN FOR NEXT-MILESTONE CONSUMPTION`

Public inventory groups:

- Identity: `TaskDefinitionId`, `TaskInstanceId`, `TaskDependencyId`, `TaskEvidenceId`, `TaskDefinitionReference`, `TaskInstanceReference`, `TaskDependencyReference`, `TaskEvidenceReference`, `TaskWorkflowReference`, `TaskStepReference`
- Definition: `TaskDefinition`, `TaskDefinitionVersion`, `TaskDefinitionName`, `TaskDescription`, `TaskKind`, `TaskInputContract`, `TaskOutputContract`, `TaskRequirement`, `TaskCapabilityRequirement`, `TaskEvidenceRequirement`, `TaskCompletionRequirement`, `TaskFailurePolicyReference`
- Instance: `TaskInstance`, `TaskDefinitionSnapshotReference`, `TaskCreationContext`, `TaskInputBinding`, `TaskOutputBinding`, `TaskWorkflowBinding`, `TaskStepBinding`, `TaskState`
- Ownership: `TaskOwner`, `TaskOwnership`, `TaskOwnershipAuthority`, `TaskOwnershipScope`
- Assignment: `TaskAssignee`, `TaskAssignment`, `TaskAssignmentAuthority`, `TaskAssignmentStatus`, `TaskAssignmentReason`, `TaskAssignmentRequest`, `TaskAssignmentChange`, `TaskAssignmentRejection`, `TaskAssignmentNoOp`, `TaskAssignmentDecision`, `TaskAssignmentControl`, `TaskAssignmentRejectionReason`
- Priority: `TaskPriority`, `TaskPriorityClass`, `TaskPriorityValue`
- Readiness: `TaskReadiness`, `TaskReadinessRequirement`, `TaskReadinessEvidence`, `TaskReadinessBlocker`, `TaskReadinessInput`, `TaskReadinessReady`, `TaskReadinessBlocked`, `TaskReadinessRejection`, `TaskReadinessDecision`, `TaskReadinessControl`, `TaskReadinessRejectionReason`
- Lifecycle: `TaskStateSnapshot`, `TaskFailureCode`, `TaskFailureCategory`, `TaskLifecycleGuards`, `TaskTransitionRequest`, `TaskAllowedTransition`, `TaskRejectedTransition`, `TaskNoOpTransition`, `TaskTransitionDecision`, `TaskTransitionControl`, `TaskTransitionRejectionReason`
- Dependency: `TaskDependencyGraphReference`, `TaskDependencySource`, `TaskDependencyTarget`, `TaskDependencyType`, `TaskDependencyRequirement`, `TaskDependencyStatus`, `TaskDependency`, `TaskDependencyFact`, `TaskDependencySet`, `TaskDependencyValidationRequest`, `TaskDependencyCoordinationRequest`, `TaskDependencyValidationAccepted`, `TaskDependencyValidationNoOp`, `TaskDependencyValidationRejected`, `TaskDependencyValidation`, `TaskDependencyDecision`, `TaskDependencyCoordinationDecision`, `TaskDependencyControl`, `TaskDependencyBlocker`, `TaskDependencyUnresolvedReason`, `TaskDependencyRejectionReason`
- Completion: `TaskOutputReference`, `TaskOutput`, `TaskCompletionResult`, `TaskCompletion`, `TaskCompletionValidationRequest`, `TaskCompletionOutcome`, `TaskCompletionControl`, `TaskCompletionRejected`, `TaskCompletionRejectionReason`
- Failure: `TaskFailureReason`, `TaskFailureReference`, `TaskRecoveryReference`, `TaskFailure`, `TaskFailureValidationRequest`, `TaskFailureOutcome`, `TaskFailureControl`, `TaskFailureRejected`, `TaskFailureRejectionReason`
- Evidence: `TaskEvidenceType`, `TaskEvidenceMetadata`, `TaskEvidence`, `TaskEvidenceSet`, `TaskEvidenceValidationRequest`, `TaskEvidenceValidation`, `TaskEvidenceControl`, `TaskEvidenceRejected`, `TaskEvidenceRejectionReason`
- Outcome: `TaskOutcomeDecision`, `TaskOutcomeRejectionReason`

Cross-module conformance:

- Happy-path completion composition remains explicit: definition -> instance -> ownership and assignment -> priority -> dependency decision -> readiness decision -> lifecycle transition -> completion acceptance -> terminal transition -> archival transition.
- Happy-path failure composition remains explicit: definition -> instance -> ownership and assignment -> readiness decision -> lifecycle transition -> failure acceptance -> terminal transition -> archival transition.
- Dependency evaluation contributes explicit facts to readiness; it does not call readiness control or mutate lifecycle.
- Readiness contributes explicit facts to lifecycle start validation; it does not mutate assignment, priority, or dependency state.
- Completion and failure validation remain separate from lifecycle mutation; lifecycle transitions consume explicit accepted facts and guard values.

Conformance guarantees:

- Public API inventory matches `crates/kernel-domain/src/task/mod.rs` and task re-exports from `crates/kernel-domain/src/lib.rs`.
- No task integration helper, flow helper, or test helper is publicly exported.
- No speculative runtime facade, scheduler, executor, repository, task manager, task engine, or orchestrator type is exported.
- Concern vocabularies remain separate: `TaskState`, `TaskAssignmentStatus`, `TaskReadinessDecision`, `TaskDependencyStatus`, `TaskCompletionOutcome`, `TaskFailureOutcome`, and `TaskOutcomeDecision` are not collapsed into one status model.
- Determinism, immutability, explicit-input-only behavior, and additive compatibility remain preserved across K7.1 through K7.9.

## K8 Integration And Freeze Review

Review status:

- `K8-001 THROUGH K8-008 IMPLEMENTED`
- `K8 IMPLEMENTATION COMPLETE`
- `K8 API FROZEN FOR NEXT-MILESTONE CONSUMPTION`

Public inventory groups:

- Identity: `ExecutionSessionId`
- Request: `ExecutionRequest`
- Context: `ExecutionContext`
- Session: `ExecutionSession`
- Outcome: `ExecutionOutcome`, `ExecutionTermination`
- Evidence: `ExecutionEvidenceBinding`
- Retry: `ExecutionRetryEligibilityDecision`, `ExecutionRetryIneligibilityReason`
- Audit: `ExecutionAuditReference`
- Validation: `ExecutionValidation`

Conformance guarantees:

- K8 public API matches additive execution re-exports from `crates/kernel-domain/src/lib.rs`.
- K1-K7 compatibility is preserved.
- K8 public API is additive only.
- No scheduler, worker dispatch, queue, process execution, network, filesystem, database, event publication, memory persistence, automatic retry execution, or automatic timeout execution is exposed.
- No task lifecycle mutation is introduced.

## K9 Enterprise Memory Freeze Review

Review status:

- `K9-001 THROUGH K9-008 IMPLEMENTED`
- `K9 IMPLEMENTATION COMPLETE`
- `K9 NATIVE VERIFICATION PASSED`
- `K9 API FROZEN FOR K10 CONSUMPTION`

Public inventory groups:

- Identity: `MemoryRecordId`, `MemoryRecordReference`
- Record: `MemoryRecord`, `MemoryProvenance`, `MemoryClassification`, `MemoryRetentionPolicyReference`, `MemoryAuditReference`
- Command and Decision: `MemoryCaptureRequest`, `MemoryCaptureDecision`, `MemoryRetentionDecision`, `MemoryRelationship`, `MemoryRelationshipRequest`, `MemoryRejectionReason`
- Retrieval: `MemoryRetrievalRequest`, `MemoryRetrievalResult`, `MemoryQuery`, `MemoryQueryResult`
- Projection: `MemoryProjection`, `WorkflowMemoryProjection`, `TaskMemoryProjection`, `ExecutionMemoryProjection`, `RuntimeMemoryProjection`

Conformance guarantees:

- K9 public API matches additive memory re-exports from `crates/kernel-domain/src/lib.rs`.
- K1-K8 compatibility is preserved.
- K9 public API is additive only.
- Classification reuses the frozen `EventClassification` vocabulary.
- Provenance remains explicit and reference-based over K5 events, K6 workflows, K7 tasks, K8 execution sessions, K4 runtime facts, K3 authorization decisions, and task evidence references.
- Retrieval and query contracts remain deterministic over supplied immutable records and projections only.
- No application service, runtime orchestration, scheduler, worker dispatch, queue, search infrastructure, network, filesystem, database, persistence, API Gateway, or frontend implementation is exposed.
- No event, workflow, task, execution, or runtime lifecycle mutation is introduced.

Freeze guarantees:

- K1-K8 API compatibility is preserved.
- K9 public API is additive only.
- Breaking changes after K9 freeze require approved ADR and compatibility review.

## K10 API Gateway Implementation Review

Review status:

- `K10-001 THROUGH K10-010 VERIFIED`
- `K10 IMPLEMENTATION COMPLETE`
- `K10 COMPILE VALIDATION PASSED`
- `K10 NATIVE VERIFICATION PASSED`
- `K10 API FROZEN FOR K11 CONSUMPTION`

Public inventory groups:

- Contract: `GatewayApiVersion`, `GatewayOperationKind`, `GatewayOperationReference`
- Authentication: `GatewayAuthenticationContext`
- Authorization: `GatewayAuthorizationBinding`
- Request: `GatewayRequestContext`, `GatewayRequestEnvelope`
- Command: `GatewayCommandPayload`, `GatewayCommandRequest`, `GatewayCommandResponse`
- Query: `GatewayQueryPayload`, `GatewayQueryRequest`, `GatewayQueryResponse`
- Response: `GatewayResponseEnvelope`
- Error: `GatewayError`, `GatewayErrorCode`, `GatewayResult`
- Protocol and Governance: `GatewayProtocol`, `GatewayRateGovernanceReference`
- Status and Audit: `GatewayStatusSnapshot`, `GatewayAuditReference`

Conformance guarantees:

- K10 public API matches additive `kernel-gateway` re-exports from `crates/kernel-gateway/src/lib.rs`.
- K1-K9 compatibility is preserved.
- K10 public API is additive only.
- Authentication remains distinct from authorization and does not integrate a provider.
- Authorization bindings delegate to frozen K3 decisions and do not re-evaluate policy.
- Request and response envelopes remain explicit, immutable, and correlation-preserving.
- Query and command contracts remain transport-neutral and side-effect free.
- No HTTP server, WebSocket, gRPC, IPC, persistence, network transport, scheduler, worker dispatch, or background runtime is exposed.

Freeze guarantees:

- K10 public API is frozen for K11 consumption.
- Breaking changes after K10 freeze require approved ADR, compatibility review, and explicit human authorization.

## K11 Studio Integration Implementation Review

Review status:

- `K11-001 THROUGH K11-010 IMPLEMENTED`
- `K11 IMPLEMENTATION COMPLETE`
- `K11 COMPILE VALIDATION PASSED`
- `K11 NATIVE VERIFICATION PASSED`
- `K11 API FROZEN FOR K12 CONSUMPTION`

Public inventory groups:

- Identity and Version: `StudioApiVersion`, `StudioViewKind`, `StudioViewReference`
- Selection and Navigation: `StudioSelectionContext`, `StudioFilterContext`, `StudioFilterReference`, `StudioSortReference`, `StudioTimeRange`, `StudioNavigationReference`
- Audit and Status: `StudioAuditReference`, `StudioStatusSnapshot`
- View Coordination: `StudioViewProjection`, `StudioViewRequest`, `StudioViewResponse`, `StudioRequestEnvelope`, `StudioResponseEnvelope`
- View Projections: `StudioTopViewProjection`, `StudioAttentionState`, `StudioDigitalTwinProjection`, `StudioRuntimeProjection`, `StudioWorkflowProjection`, `StudioTaskProjection`, `StudioEventTimelineProjection`, `StudioMemoryProjection`, `StudioAuditProjection`, `StudioRevenueReferenceProjection`
- Command Console: `StudioCommandRequest`, `StudioCommandResponse`
- Error Model: `StudioError`, `StudioErrorCode`, `StudioResult`

Conformance guarantees:

- K11 public API matches additive `kernel-studio` re-exports from `crates/kernel-studio/src/lib.rs`.
- K1-K10 compatibility is preserved.
- K11 public API is additive only.
- Studio request and response contracts remain technology-neutral, transport-neutral, immutable, and side-effect free.
- Studio commands and reads preserve the frozen K10 gateway boundary and do not bypass authorization, execution, or memory governance.
- No HTML, CSS, JavaScript, browser state, desktop runtime, network transport, persistence, scheduler, worker runtime, or authentication-provider integration is exposed.

Freeze guarantees:

- K11 public API is frozen for K12 consumption.
- Breaking changes after K11 freeze require approved ADR, compatibility review, and explicit human authorization.

## K12 Application Integration Implementation Review

Review status:

- `K12-001 K12-002 K12-003 K12-004 K12-006 K12-007 K12-009 K12-010 IMPLEMENTED`
- `K12 IMPLEMENTATION COMPLETE`
- `K12 COMPILE VALIDATION PASSED`
- `K12 NATIVE VERIFICATION PASSED`
- `K12 API FROZEN FOR K13 CONSUMPTION`

Public inventory groups:

- Identity and Version: `ApplicationApiVersion`, `ApplicationIdentity`, `ApplicationIdentityKind`
- Capability and Admission: `ApplicationCapabilityReference`, `ApplicationCapabilityDeclaration`
- Context and Session: `ApplicationRequestId`, `ApplicationAuditReference`, `ApplicationRequestContext`, `ApplicationSessionReference`, `ApplicationSessionStatusReference`
- Navigation and Intent: `ApplicationIntentKind`, `ApplicationViewIntent`, `ApplicationCommandIntent`, `ApplicationQueryIntent`, `ApplicationRequestEnvelope`
- Response and Error: `ApplicationResponseKind`, `ApplicationResponseStatusReference`, `ApplicationResponsePayload`, `ApplicationResponseEnvelope`, `ApplicationError`, `ApplicationErrorCode`, `ApplicationResult`
- Status and Compatibility: `ApplicationDependencyCompatibilityReference`, `ApplicationStatusSnapshot`, `ApplicationValidationStatus`

Conformance guarantees:

- K12 public API matches additive `kernel-application` re-exports from `crates/kernel-application/src/lib.rs`.
- K1-K11 compatibility is preserved.
- K12 public API is additive only.
- Application contracts remain technology-neutral, transport-neutral, runtime-free, infrastructure-free, immutable, and side-effect free.
- K12 command and query intent remains coordinated through frozen K11 Studio requests and frozen K10 gateway evidence only.
- `kernel-application -> kernel-studio` remains the primary dependency direction.
- Direct `kernel-gateway` and `kernel-domain` dependencies are exceptional and are limited to frozen K10 authentication and authorization references plus frozen value types that are unavailable through `kernel-studio`.
- No HTTP server, WebSocket, REST routing, IPC hosting, async runtime, persistence, session storage, cache, scheduler, worker runtime, authentication provider, browser runtime, or desktop runtime is exposed.

Freeze guarantees:

- K12 public API is frozen for K13 consumption.
- Breaking changes after K12 freeze require approved ADR, compatibility review, and explicit human authorization.

## K13 Service Integration Implementation Review

Review status:

- `K13-001 THROUGH K13-010 IMPLEMENTED`
- `K13 IMPLEMENTATION COMPLETE`
- `K13 WORKSPACE INTEGRATION PASSED`
- `K13 COMPILE VALIDATION PASSED`
- `K13 NATIVE VERIFICATION PASSED`
- `K13 API FROZEN FOR K14 CONSUMPTION`

Public inventory groups:

- Identity and Version: `ServiceApiVersion`, `ServiceIdentity`, `ServiceIdentityKind`
- Capability and Admission: `ServiceCapabilityReference`, `ServiceCapabilityDeclaration`, `SERVICE_COMMAND_CAPABILITY`, `SERVICE_QUERY_CAPABILITY`
- Request and Intent: `ServiceIntentKind`, `ServiceRequestId`, `ServiceRequestContext`, `ServiceCommandIntent`, `ServiceQueryIntent`
- Response and Error: `ServiceResponseKind`, `ServiceResponseStatusReference`, `ServiceResponseEnvelope`, `ServiceError`, `ServiceErrorCode`, `ServiceResult`
- Status and Compatibility: `ServiceDependencyCompatibilityReference`, `ServiceStatusSnapshot`, `ServiceValidationStatus`

Conformance guarantees:

- K13 public API matches additive `kernel-service` re-exports from `crates/kernel-service/src/lib.rs`.
- K1-K12 compatibility is preserved.
- K13 public API is additive only.
- Service contracts remain technology-neutral, transport-neutral, runtime-free, infrastructure-free, immutable, and side-effect free.
- `kernel-service -> kernel-application` remains the primary production dependency direction.
- Lower-layer `kernel-domain`, `kernel-gateway`, and `kernel-studio` dependencies are limited to test-only `dev-dependencies` for service fixtures.
- K13 does not bypass `kernel-application`, `kernel-studio`, or `kernel-gateway`, and does not mutate `kernel-domain` directly.
- No runtime, Tokio, networking, transport, persistence, database, scheduler, queue, filesystem behavior, cache, plugin loader, AI model execution, or infrastructure is exposed.

Freeze guarantees:

- K13 public API is frozen for K14 consumption.
- Any incompatible K13 public API change requires an approved ADR.
