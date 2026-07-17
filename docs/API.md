# API

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-17

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

This document records the current public K6 workflow API and the additive K7.1 task-foundation API exposed from `crates/kernel-domain/src/lib.rs`. K6 remains frozen. K7-001 is implemented, deterministic, and pending API acceptance.

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

- `IMPLEMENTED — PENDING API ACCEPTANCE`
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

### Workflow-Related Domain Errors

Public variants:

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
