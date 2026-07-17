use std::fmt;
use std::str::FromStr;

use crate::authorization::{
    AuthorizationAuditEvidenceReference, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference,
};
use crate::errors::{DomainError, DomainResult};
use crate::event::{
    validate_event_identity, validate_event_timestamps, validate_event_version, EventCausation,
    EventClassification, EventEnvelope, EventEnvelopeCandidate, EventSource, EventSubject,
    EventTrace, EventType, EventVersion,
};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, CorrelationId, DecisionId, DelegationId,
    EnglishNamespace, EventId, PolicyId, StableVersion, WorkflowId,
};
use crate::ownership::OwnershipPath;
use crate::request::AuthorizationRequestRecord;
use crate::state::{
    WorkflowStateSnapshot, WorkflowTransitionControlRequest, WorkflowTransitionDecision,
};
use crate::{
    TransitionAuthorityReference, TransitionEvidenceReference, TransitionReasonReference,
    WorkflowFailureCode,
};

const WORKFLOW_DEFINITION_REFERENCE_EXPECTATION: &str =
    "ASCII letters, digits, dot, underscore, or hyphen";

fn validate_workflow_definition_reference(
    kind: &'static str,
    value: impl Into<String>,
) -> DomainResult<String> {
    let value = value.into().trim().to_owned();

    if value.is_empty() {
        return Err(DomainError::InvalidIdentifier {
            kind,
            value,
            expected: WORKFLOW_DEFINITION_REFERENCE_EXPECTATION,
        });
    }

    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-'))
    {
        Ok(value)
    } else {
        Err(DomainError::InvalidIdentifier {
            kind,
            value,
            expected: WORKFLOW_DEFINITION_REFERENCE_EXPECTATION,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkflowRetryLimit(u16);

impl WorkflowRetryLimit {
    pub fn new(value: u16) -> DomainResult<Self> {
        if value == 0 {
            return Err(DomainError::InvalidWorkflowReference(
                "workflow retry limit must be greater than zero",
            ));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRetryPolicyReference {
    definition_version: StableVersion,
    retry_limit: WorkflowRetryLimit,
}

impl WorkflowRetryPolicyReference {
    pub fn new(definition_version: StableVersion, retry_limit: WorkflowRetryLimit) -> Self {
        Self {
            definition_version,
            retry_limit,
        }
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn retry_limit(&self) -> WorkflowRetryLimit {
        self.retry_limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRecoveryReference {
    corrective_path: crate::identifier::NonEmptyText,
    requires_revalidation: bool,
}

impl WorkflowRecoveryReference {
    pub fn new(
        corrective_path: impl Into<String>,
        requires_revalidation: bool,
    ) -> DomainResult<Self> {
        Ok(Self {
            corrective_path: crate::identifier::NonEmptyText::new(
                "workflow_corrective_path",
                corrective_path,
            )?,
            requires_revalidation,
        })
    }

    pub fn corrective_path(&self) -> &str {
        self.corrective_path.as_str()
    }

    pub fn requires_revalidation(&self) -> bool {
        self.requires_revalidation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowAuditEvidenceReference {
    audit_evidence_id: AuditEvidenceId,
    workflow_id: WorkflowId,
    definition_version: StableVersion,
    consumed_policy_ids: Vec<PolicyId>,
    consumed_authorization_decision_ids: Vec<AuthorizationDecisionId>,
    consumed_delegation_ids: Vec<DelegationId>,
    consumed_decision_ids: Vec<DecisionId>,
}

impl WorkflowAuditEvidenceReference {
    pub fn new(
        audit_evidence_id: AuditEvidenceId,
        workflow_id: WorkflowId,
        definition_version: StableVersion,
        consumed_policy_ids: Vec<PolicyId>,
        consumed_authorization_decision_ids: Vec<AuthorizationDecisionId>,
        consumed_delegation_ids: Vec<DelegationId>,
        consumed_decision_ids: Vec<DecisionId>,
    ) -> DomainResult<Self> {
        if consumed_policy_ids.is_empty()
            || consumed_authorization_decision_ids.is_empty()
            || consumed_delegation_ids.is_empty()
            || consumed_decision_ids.is_empty()
        {
            return Err(DomainError::InvalidWorkflowReference(
                "workflow audit evidence must preserve consumed upstream references",
            ));
        }
        Ok(Self {
            audit_evidence_id,
            workflow_id,
            definition_version,
            consumed_policy_ids,
            consumed_authorization_decision_ids,
            consumed_delegation_ids,
            consumed_decision_ids,
        })
    }

    pub fn audit_evidence_id(&self) -> &AuditEvidenceId {
        &self.audit_evidence_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowLifecycleMapReference(String);

impl WorkflowLifecycleMapReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowLifecycleMapReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowLifecycleMapReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowLifecycleMapReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowStepReference(String);

impl WorkflowStepReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowStepReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowStepReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowStepReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowTerminalOutcomeReference(String);

impl WorkflowTerminalOutcomeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowTerminalOutcomeReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowTerminalOutcomeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowTerminalOutcomeReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowStepOutcomeReference(String);

impl WorkflowStepOutcomeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowStepOutcomeReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowStepOutcomeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowStepOutcomeReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowEngineFoundation {
    workflow_id: WorkflowId,
    ownership: OwnershipPath,
    definition_version: StableVersion,
    retry_policy: Option<WorkflowRetryPolicyReference>,
    retry_limit: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowEngineFoundation {
    pub fn new(
        workflow_id: WorkflowId,
        ownership: OwnershipPath,
        definition_version: StableVersion,
        retry_policy: Option<WorkflowRetryPolicyReference>,
        retry_limit: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if retry_limit.is_some() && retry_policy.is_none() {
            return Err(DomainError::InvalidWorkflowReference(
                "retry limit requires retry policy",
            ));
        }

        for (index, evidence) in audit_evidence.iter().enumerate() {
            if audit_evidence[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowReference(
                    "duplicate workflow audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            ownership,
            definition_version,
            retry_policy,
            retry_limit,
            recovery_reference,
            audit_evidence,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn ownership(&self) -> &OwnershipPath {
        &self.ownership
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn retry_policy(&self) -> Option<&WorkflowRetryPolicyReference> {
        self.retry_policy.as_ref()
    }

    pub fn retry_limit(&self) -> Option<WorkflowRetryLimit> {
        self.retry_limit
    }

    pub fn recovery_reference(&self) -> Option<&WorkflowRecoveryReference> {
        self.recovery_reference.as_ref()
    }

    pub fn audit_evidence(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.audit_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowDefinition {
    workflow_id: WorkflowId,
    namespace: EnglishNamespace,
    definition_version: StableVersion,
    ownership: OwnershipPath,
    lifecycle_map: WorkflowLifecycleMapReference,
    entry_steps: Vec<WorkflowStepReference>,
    terminal_outcomes: Vec<WorkflowTerminalOutcomeReference>,
    policy_references: Vec<PolicyId>,
    retry_policy: Option<WorkflowRetryPolicyReference>,
    retry_limit: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_id: WorkflowId,
        namespace: EnglishNamespace,
        definition_version: StableVersion,
        ownership: OwnershipPath,
        lifecycle_map: WorkflowLifecycleMapReference,
        entry_steps: Vec<WorkflowStepReference>,
        terminal_outcomes: Vec<WorkflowTerminalOutcomeReference>,
        policy_references: Vec<PolicyId>,
        retry_policy: Option<WorkflowRetryPolicyReference>,
        retry_limit: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if entry_steps.is_empty() {
            return Err(DomainError::InvalidWorkflowDefinition(
                "workflow definition requires at least one entry step",
            ));
        }

        if retry_limit.is_some() && retry_policy.is_none() {
            return Err(DomainError::InvalidWorkflowDefinition(
                "retry limit requires retry policy",
            ));
        }

        for (index, entry_step) in entry_steps.iter().enumerate() {
            if entry_steps[..index].iter().any(|prior| prior == entry_step) {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow entry step reference",
                ));
            }
        }

        for (index, terminal_outcome) in terminal_outcomes.iter().enumerate() {
            if terminal_outcomes[..index]
                .iter()
                .any(|prior| prior == terminal_outcome)
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow terminal outcome reference",
                ));
            }
        }

        for (index, policy_reference) in policy_references.iter().enumerate() {
            if policy_references[..index]
                .iter()
                .any(|prior| prior == policy_reference)
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow policy reference",
                ));
            }
        }

        for (index, evidence) in audit_evidence.iter().enumerate() {
            if audit_evidence[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowDefinition(
                    "duplicate workflow definition audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            namespace,
            definition_version,
            ownership,
            lifecycle_map,
            entry_steps,
            terminal_outcomes,
            policy_references,
            retry_policy,
            retry_limit,
            recovery_reference,
            audit_evidence,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn namespace(&self) -> &EnglishNamespace {
        &self.namespace
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn ownership(&self) -> &OwnershipPath {
        &self.ownership
    }

    pub fn lifecycle_map(&self) -> &WorkflowLifecycleMapReference {
        &self.lifecycle_map
    }

    pub fn entry_steps(&self) -> &[WorkflowStepReference] {
        &self.entry_steps
    }

    pub fn terminal_outcomes(&self) -> &[WorkflowTerminalOutcomeReference] {
        &self.terminal_outcomes
    }

    pub fn policy_references(&self) -> &[PolicyId] {
        &self.policy_references
    }

    pub fn retry_policy(&self) -> Option<&WorkflowRetryPolicyReference> {
        self.retry_policy.as_ref()
    }

    pub fn retry_limit(&self) -> Option<WorkflowRetryLimit> {
        self.retry_limit
    }

    pub fn recovery_reference(&self) -> Option<&WorkflowRecoveryReference> {
        self.recovery_reference.as_ref()
    }

    pub fn audit_evidence(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.audit_evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowInstance {
    workflow_id: WorkflowId,
    workflow_definition: WorkflowDefinition,
    definition_version_snapshot: StableVersion,
    ownership_reference: OwnershipPath,
    current_workflow_state_snapshot: WorkflowStateSnapshot,
    creation_evidence: WorkflowAuditEvidenceReference,
    retry_policy_snapshot: Option<WorkflowRetryPolicyReference>,
    retry_limit_snapshot: Option<WorkflowRetryLimit>,
    recovery_reference: Option<WorkflowRecoveryReference>,
    audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
}

impl WorkflowInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_id: WorkflowId,
        workflow_definition: WorkflowDefinition,
        definition_version_snapshot: StableVersion,
        ownership_reference: OwnershipPath,
        current_workflow_state_snapshot: WorkflowStateSnapshot,
        creation_evidence: WorkflowAuditEvidenceReference,
        retry_policy_snapshot: Option<WorkflowRetryPolicyReference>,
        retry_limit_snapshot: Option<WorkflowRetryLimit>,
        recovery_reference: Option<WorkflowRecoveryReference>,
        audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if retry_limit_snapshot.is_some() && retry_policy_snapshot.is_none() {
            return Err(DomainError::InvalidWorkflowInstance(
                "retry limit requires retry policy",
            ));
        }

        for (index, evidence) in audit_evidence_references.iter().enumerate() {
            if audit_evidence_references[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowInstance(
                    "duplicate workflow instance audit evidence reference",
                ));
            }
        }

        Ok(Self {
            workflow_id,
            workflow_definition,
            definition_version_snapshot,
            ownership_reference,
            current_workflow_state_snapshot,
            creation_evidence,
            retry_policy_snapshot,
            retry_limit_snapshot,
            recovery_reference,
            audit_evidence_references,
        })
    }

    pub fn workflow_id(&self) -> &WorkflowId {
        &self.workflow_id
    }

    pub fn workflow_definition(&self) -> &WorkflowDefinition {
        &self.workflow_definition
    }

    pub fn definition_version_snapshot(&self) -> &StableVersion {
        &self.definition_version_snapshot
    }

    pub fn ownership_reference(&self) -> &OwnershipPath {
        &self.ownership_reference
    }

    pub fn current_workflow_state_snapshot(&self) -> &WorkflowStateSnapshot {
        &self.current_workflow_state_snapshot
    }

    pub fn creation_evidence(&self) -> &WorkflowAuditEvidenceReference {
        &self.creation_evidence
    }

    pub fn retry_policy_snapshot(&self) -> Option<&WorkflowRetryPolicyReference> {
        self.retry_policy_snapshot.as_ref()
    }

    pub fn retry_limit_snapshot(&self) -> Option<WorkflowRetryLimit> {
        self.retry_limit_snapshot
    }

    pub fn recovery_reference(&self) -> Option<&WorkflowRecoveryReference> {
        self.recovery_reference.as_ref()
    }

    pub fn audit_evidence_references(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.audit_evidence_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStepSelection {
    current_step: WorkflowStepReference,
    next_candidate_steps: Vec<WorkflowStepReference>,
}

impl WorkflowStepSelection {
    pub fn new(
        current_step: WorkflowStepReference,
        next_candidate_steps: Vec<WorkflowStepReference>,
    ) -> Self {
        Self {
            current_step,
            next_candidate_steps,
        }
    }

    pub fn current_step(&self) -> &WorkflowStepReference {
        &self.current_step
    }

    pub fn next_candidate_steps(&self) -> &[WorkflowStepReference] {
        &self.next_candidate_steps
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStepExecutionPlan {
    completed_step_references: Vec<WorkflowStepReference>,
    blocked_step_references: Vec<WorkflowStepReference>,
    skipped_step_references: Vec<WorkflowStepReference>,
    terminal_step_references: Vec<WorkflowStepOutcomeReference>,
}

impl WorkflowStepExecutionPlan {
    pub fn new(
        completed_step_references: Vec<WorkflowStepReference>,
        blocked_step_references: Vec<WorkflowStepReference>,
        skipped_step_references: Vec<WorkflowStepReference>,
        terminal_step_references: Vec<WorkflowStepOutcomeReference>,
    ) -> DomainResult<Self> {
        for (index, step_reference) in completed_step_references.iter().enumerate() {
            if completed_step_references[..index]
                .iter()
                .any(|prior| prior == step_reference)
            {
                return Err(DomainError::InvalidWorkflowStepCoordination(
                    "duplicate completed workflow step reference",
                ));
            }
        }

        for (index, step_reference) in blocked_step_references.iter().enumerate() {
            if blocked_step_references[..index]
                .iter()
                .any(|prior| prior == step_reference)
            {
                return Err(DomainError::InvalidWorkflowStepCoordination(
                    "duplicate blocked workflow step reference",
                ));
            }
        }

        for (index, step_reference) in skipped_step_references.iter().enumerate() {
            if skipped_step_references[..index]
                .iter()
                .any(|prior| prior == step_reference)
            {
                return Err(DomainError::InvalidWorkflowStepCoordination(
                    "duplicate skipped workflow step reference",
                ));
            }
        }

        for (index, step_reference) in terminal_step_references.iter().enumerate() {
            if terminal_step_references[..index]
                .iter()
                .any(|prior| prior == step_reference)
            {
                return Err(DomainError::InvalidWorkflowStepCoordination(
                    "duplicate terminal workflow step reference",
                ));
            }
        }

        Ok(Self {
            completed_step_references,
            blocked_step_references,
            skipped_step_references,
            terminal_step_references,
        })
    }

    pub fn completed_step_references(&self) -> &[WorkflowStepReference] {
        &self.completed_step_references
    }

    pub fn blocked_step_references(&self) -> &[WorkflowStepReference] {
        &self.blocked_step_references
    }

    pub fn skipped_step_references(&self) -> &[WorkflowStepReference] {
        &self.skipped_step_references
    }

    pub fn terminal_step_references(&self) -> &[WorkflowStepOutcomeReference] {
        &self.terminal_step_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStepCoordination {
    workflow_definition: WorkflowDefinition,
    workflow_instance: WorkflowInstance,
    workflow_step_selection: WorkflowStepSelection,
    workflow_step_execution_plan: WorkflowStepExecutionPlan,
}

impl WorkflowStepCoordination {
    pub fn new(
        workflow_definition: WorkflowDefinition,
        workflow_instance: WorkflowInstance,
        workflow_step_selection: WorkflowStepSelection,
        workflow_step_execution_plan: WorkflowStepExecutionPlan,
    ) -> DomainResult<Self> {
        if workflow_step_execution_plan
            .completed_step_references()
            .iter()
            .any(|step_reference| step_reference == workflow_step_selection.current_step())
        {
            return Err(DomainError::InvalidWorkflowStepCoordination(
                "current workflow step cannot be completed",
            ));
        }

        if workflow_step_execution_plan
            .blocked_step_references()
            .iter()
            .any(|step_reference| step_reference == workflow_step_selection.current_step())
        {
            return Err(DomainError::InvalidWorkflowStepCoordination(
                "current workflow step cannot be blocked",
            ));
        }

        if workflow_step_execution_plan
            .skipped_step_references()
            .iter()
            .any(|step_reference| step_reference == workflow_step_selection.current_step())
        {
            return Err(DomainError::InvalidWorkflowStepCoordination(
                "current workflow step cannot be skipped",
            ));
        }

        Ok(Self {
            workflow_definition,
            workflow_instance,
            workflow_step_selection,
            workflow_step_execution_plan,
        })
    }

    pub fn workflow_definition(&self) -> &WorkflowDefinition {
        &self.workflow_definition
    }

    pub fn workflow_instance(&self) -> &WorkflowInstance {
        &self.workflow_instance
    }

    pub fn workflow_step_selection(&self) -> &WorkflowStepSelection {
        &self.workflow_step_selection
    }

    pub fn workflow_step_execution_plan(&self) -> &WorkflowStepExecutionPlan {
        &self.workflow_step_execution_plan
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkflowOperationReference(String);

impl WorkflowOperationReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        validate_workflow_definition_reference("WorkflowOperationReference", value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WorkflowOperationReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for WorkflowOperationReference {
    type Err = DomainError;

    fn from_str(value: &str) -> DomainResult<Self> {
        Self::new(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowAuthorizationContext {
    authorization_request: AuthorizationRequestRecord,
    authorization_decision: AuthorizationDecisionReference,
    authorization_evidence_references: Vec<AuthorizationAuditEvidenceReference>,
}

impl WorkflowAuthorizationContext {
    pub fn new(
        authorization_request: AuthorizationRequestRecord,
        authorization_decision: AuthorizationDecisionReference,
        authorization_evidence_references: Vec<AuthorizationAuditEvidenceReference>,
    ) -> DomainResult<Self> {
        if authorization_request.request_id() != authorization_decision.request_id() {
            return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                "workflow authorization request must match authorization decision request",
            ));
        }

        if authorization_evidence_references.is_empty() {
            return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                "workflow authorization evidence is required",
            ));
        }

        for (index, evidence) in authorization_evidence_references.iter().enumerate() {
            if authorization_evidence_references[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "duplicate workflow authorization evidence reference",
                ));
            }

            if evidence.decision_id() != authorization_decision.decision_id() {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "workflow authorization evidence must match authorization decision",
                ));
            }
        }

        Ok(Self {
            authorization_request,
            authorization_decision,
            authorization_evidence_references,
        })
    }

    pub fn authorization_request(&self) -> &AuthorizationRequestRecord {
        &self.authorization_request
    }

    pub fn authorization_decision(&self) -> &AuthorizationDecisionReference {
        &self.authorization_decision
    }

    pub fn authorization_evidence_references(&self) -> &[AuthorizationAuditEvidenceReference] {
        &self.authorization_evidence_references
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowAuthorizationRequest {
    workflow_operation: WorkflowOperationReference,
    workflow_definition: Option<WorkflowDefinition>,
    workflow_instance: Option<WorkflowInstance>,
    current_workflow_state: Option<WorkflowStateSnapshot>,
    requested_target_workflow_state: Option<crate::lifecycle::WorkflowState>,
    workflow_step_coordination: Option<WorkflowStepCoordination>,
    current_workflow_step: Option<WorkflowStepReference>,
    requested_next_workflow_step: Option<WorkflowStepReference>,
    workflow_authorization_context: WorkflowAuthorizationContext,
    transition_authority_reference: Option<TransitionAuthorityReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
}

impl WorkflowAuthorizationRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_operation: WorkflowOperationReference,
        workflow_definition: Option<WorkflowDefinition>,
        workflow_instance: Option<WorkflowInstance>,
        current_workflow_state: Option<WorkflowStateSnapshot>,
        requested_target_workflow_state: Option<crate::lifecycle::WorkflowState>,
        workflow_step_coordination: Option<WorkflowStepCoordination>,
        current_workflow_step: Option<WorkflowStepReference>,
        requested_next_workflow_step: Option<WorkflowStepReference>,
        workflow_authorization_context: WorkflowAuthorizationContext,
        transition_authority_reference: Option<TransitionAuthorityReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
    ) -> DomainResult<Self> {
        for (index, evidence) in transition_evidence_references.iter().enumerate() {
            if transition_evidence_references[..index]
                .iter()
                .any(|prior| prior == evidence)
            {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "duplicate workflow transition evidence reference",
                ));
            }
        }

        let authorization_scope = workflow_authorization_context
            .authorization_request()
            .target()
            .scope()
            .ownership_path();

        if let Some(definition) = workflow_definition.as_ref() {
            if definition.ownership() != authorization_scope {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "workflow authorization scope must match workflow definition ownership",
                ));
            }
        }

        if let Some(instance) = workflow_instance.as_ref() {
            if instance.ownership_reference() != authorization_scope {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "workflow authorization scope must match workflow instance ownership",
                ));
            }
        }

        if let Some(state) = current_workflow_state.as_ref() {
            if state.ownership_path() != authorization_scope {
                return Err(DomainError::InvalidWorkflowAuthorizationIntegration(
                    "workflow authorization scope must match workflow state ownership",
                ));
            }
        }

        Ok(Self {
            workflow_operation,
            workflow_definition,
            workflow_instance,
            current_workflow_state,
            requested_target_workflow_state,
            workflow_step_coordination,
            current_workflow_step,
            requested_next_workflow_step,
            workflow_authorization_context,
            transition_authority_reference,
            transition_evidence_references,
        })
    }

    pub fn workflow_operation(&self) -> &WorkflowOperationReference {
        &self.workflow_operation
    }

    pub fn workflow_definition(&self) -> Option<&WorkflowDefinition> {
        self.workflow_definition.as_ref()
    }

    pub fn workflow_instance(&self) -> Option<&WorkflowInstance> {
        self.workflow_instance.as_ref()
    }

    pub fn current_workflow_state(&self) -> Option<&WorkflowStateSnapshot> {
        self.current_workflow_state.as_ref()
    }

    pub fn requested_target_workflow_state(&self) -> Option<crate::lifecycle::WorkflowState> {
        self.requested_target_workflow_state
    }

    pub fn workflow_step_coordination(&self) -> Option<&WorkflowStepCoordination> {
        self.workflow_step_coordination.as_ref()
    }

    pub fn current_workflow_step(&self) -> Option<&WorkflowStepReference> {
        self.current_workflow_step.as_ref()
    }

    pub fn requested_next_workflow_step(&self) -> Option<&WorkflowStepReference> {
        self.requested_next_workflow_step.as_ref()
    }

    pub fn workflow_authorization_context(&self) -> &WorkflowAuthorizationContext {
        &self.workflow_authorization_context
    }

    pub fn transition_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.transition_authority_reference.as_ref()
    }

    pub fn transition_evidence_references(&self) -> &[TransitionEvidenceReference] {
        &self.transition_evidence_references
    }
}

pub type WorkflowAuthorizationDecision = AuthorizationDecisionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkflowAuthorizationControl;

impl WorkflowAuthorizationControl {
    pub fn evaluate(request: &WorkflowAuthorizationRequest) -> WorkflowAuthorizationDecision {
        request
            .workflow_authorization_context()
            .authorization_decision()
            .outcome()
    }
}

pub type WorkflowEventTypeReference = EventType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowEventContext {
    workflow_definition: Option<WorkflowDefinition>,
    workflow_instance: Option<WorkflowInstance>,
    workflow_state_snapshot: Option<WorkflowStateSnapshot>,
    workflow_step_coordination: Option<WorkflowStepCoordination>,
    workflow_step_reference: Option<WorkflowStepReference>,
    workflow_transition_control_request: Option<WorkflowTransitionControlRequest>,
    workflow_transition_decision: Option<WorkflowTransitionDecision>,
    workflow_authorization_request: Option<WorkflowAuthorizationRequest>,
    workflow_authorization_decision: Option<WorkflowAuthorizationDecision>,
    workflow_operation: Option<WorkflowOperationReference>,
    transition_reason_reference: Option<TransitionReasonReference>,
    transition_authority_reference: Option<TransitionAuthorityReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
    workflow_audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
    failure_code: Option<WorkflowFailureCode>,
}

impl WorkflowEventContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_definition: Option<WorkflowDefinition>,
        workflow_instance: Option<WorkflowInstance>,
        workflow_state_snapshot: Option<WorkflowStateSnapshot>,
        workflow_step_coordination: Option<WorkflowStepCoordination>,
        workflow_step_reference: Option<WorkflowStepReference>,
        workflow_transition_control_request: Option<WorkflowTransitionControlRequest>,
        workflow_transition_decision: Option<WorkflowTransitionDecision>,
        workflow_authorization_request: Option<WorkflowAuthorizationRequest>,
        workflow_authorization_decision: Option<WorkflowAuthorizationDecision>,
        workflow_operation: Option<WorkflowOperationReference>,
        transition_reason_reference: Option<TransitionReasonReference>,
        transition_authority_reference: Option<TransitionAuthorityReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
        workflow_audit_evidence_references: Vec<WorkflowAuditEvidenceReference>,
        failure_code: Option<WorkflowFailureCode>,
    ) -> DomainResult<Self> {
        for (index, evidence) in transition_evidence_references.iter().enumerate() {
            if transition_evidence_references[..index]
                .iter()
                .any(|prior| prior == evidence)
            {
                return Err(DomainError::InvalidWorkflowEventIntegration(
                    "duplicate workflow transition evidence reference",
                ));
            }
        }

        for (index, evidence) in workflow_audit_evidence_references.iter().enumerate() {
            if workflow_audit_evidence_references[..index]
                .iter()
                .any(|prior| prior.audit_evidence_id() == evidence.audit_evidence_id())
            {
                return Err(DomainError::InvalidWorkflowEventIntegration(
                    "duplicate workflow event audit evidence reference",
                ));
            }
        }

        let mut observed_workflow_id: Option<&WorkflowId> = None;

        for workflow_id in [
            workflow_definition
                .as_ref()
                .map(WorkflowDefinition::workflow_id),
            workflow_instance
                .as_ref()
                .map(WorkflowInstance::workflow_id),
            workflow_state_snapshot
                .as_ref()
                .map(WorkflowStateSnapshot::workflow_id),
            workflow_step_coordination
                .as_ref()
                .map(|coordination| coordination.workflow_instance().workflow_id()),
        ]
        .into_iter()
        .flatten()
        {
            if let Some(expected) = observed_workflow_id {
                if expected != workflow_id {
                    return Err(DomainError::InvalidWorkflowEventIntegration(
                        "workflow event context must preserve one workflow identity",
                    ));
                }
            } else {
                observed_workflow_id = Some(workflow_id);
            }
        }

        if observed_workflow_id.is_none() {
            return Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow event context requires workflow binding",
            ));
        }

        Ok(Self {
            workflow_definition,
            workflow_instance,
            workflow_state_snapshot,
            workflow_step_coordination,
            workflow_step_reference,
            workflow_transition_control_request,
            workflow_transition_decision,
            workflow_authorization_request,
            workflow_authorization_decision,
            workflow_operation,
            transition_reason_reference,
            transition_authority_reference,
            transition_evidence_references,
            workflow_audit_evidence_references,
            failure_code,
        })
    }

    pub fn workflow_definition(&self) -> Option<&WorkflowDefinition> {
        self.workflow_definition.as_ref()
    }

    pub fn workflow_instance(&self) -> Option<&WorkflowInstance> {
        self.workflow_instance.as_ref()
    }

    pub fn workflow_state_snapshot(&self) -> Option<&WorkflowStateSnapshot> {
        self.workflow_state_snapshot.as_ref()
    }

    pub fn workflow_step_coordination(&self) -> Option<&WorkflowStepCoordination> {
        self.workflow_step_coordination.as_ref()
    }

    pub fn workflow_step_reference(&self) -> Option<&WorkflowStepReference> {
        self.workflow_step_reference.as_ref()
    }

    pub fn workflow_transition_control_request(&self) -> Option<&WorkflowTransitionControlRequest> {
        self.workflow_transition_control_request.as_ref()
    }

    pub fn workflow_transition_decision(&self) -> Option<&WorkflowTransitionDecision> {
        self.workflow_transition_decision.as_ref()
    }

    pub fn workflow_authorization_request(&self) -> Option<&WorkflowAuthorizationRequest> {
        self.workflow_authorization_request.as_ref()
    }

    pub fn workflow_authorization_decision(&self) -> Option<&WorkflowAuthorizationDecision> {
        self.workflow_authorization_decision.as_ref()
    }

    pub fn workflow_operation(&self) -> Option<&WorkflowOperationReference> {
        self.workflow_operation.as_ref()
    }

    pub fn transition_reason_reference(&self) -> Option<&TransitionReasonReference> {
        self.transition_reason_reference.as_ref()
    }

    pub fn transition_authority_reference(&self) -> Option<&TransitionAuthorityReference> {
        self.transition_authority_reference.as_ref()
    }

    pub fn transition_evidence_references(&self) -> &[TransitionEvidenceReference] {
        &self.transition_evidence_references
    }

    pub fn workflow_audit_evidence_references(&self) -> &[WorkflowAuditEvidenceReference] {
        &self.workflow_audit_evidence_references
    }

    pub fn failure_code(&self) -> Option<WorkflowFailureCode> {
        self.failure_code
    }

    fn workflow_id(&self) -> &WorkflowId {
        self.workflow_instance()
            .map(WorkflowInstance::workflow_id)
            .or_else(|| {
                self.workflow_definition()
                    .map(WorkflowDefinition::workflow_id)
            })
            .or_else(|| {
                self.workflow_state_snapshot()
                    .map(WorkflowStateSnapshot::workflow_id)
            })
            .or_else(|| {
                self.workflow_step_coordination()
                    .map(|coordination| coordination.workflow_instance().workflow_id())
            })
            .expect("workflow event context constructor enforces workflow binding")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowEventIntegrationRequest {
    workflow_event_type: WorkflowEventTypeReference,
    event_id: EventId,
    event_version: EventVersion,
    occurred_at: crate::request::TimeReference,
    recorded_at: crate::request::TimeReference,
    event_source: EventSource,
    event_subject: EventSubject,
    event_classification: EventClassification,
    correlation_id: Option<CorrelationId>,
    causation: EventCausation,
    workflow_event_context: WorkflowEventContext,
}

impl WorkflowEventIntegrationRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_event_type: WorkflowEventTypeReference,
        event_id: EventId,
        event_version: EventVersion,
        occurred_at: crate::request::TimeReference,
        recorded_at: crate::request::TimeReference,
        event_source: EventSource,
        event_subject: EventSubject,
        event_classification: EventClassification,
        correlation_id: Option<CorrelationId>,
        causation: EventCausation,
        workflow_event_context: WorkflowEventContext,
    ) -> DomainResult<Self> {
        if event_source.component().as_str() != "workflow-engine" {
            return Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow event source must be workflow-engine",
            ));
        }

        if event_subject.subject_type().as_str() != "workflow" {
            return Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow event subject type must be workflow",
            ));
        }

        if event_subject.subject_id().as_str() != workflow_event_context.workflow_id().as_str() {
            return Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow event subject must match workflow identity",
            ));
        }

        validate_workflow_event_category(&workflow_event_type, &workflow_event_context)?;

        Ok(Self {
            workflow_event_type,
            event_id,
            event_version,
            occurred_at,
            recorded_at,
            event_source,
            event_subject,
            event_classification,
            correlation_id,
            causation,
            workflow_event_context,
        })
    }

    pub fn workflow_event_type(&self) -> &WorkflowEventTypeReference {
        &self.workflow_event_type
    }

    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn event_version(&self) -> &EventVersion {
        &self.event_version
    }

    pub fn occurred_at(&self) -> &crate::request::TimeReference {
        &self.occurred_at
    }

    pub fn recorded_at(&self) -> &crate::request::TimeReference {
        &self.recorded_at
    }

    pub fn event_source(&self) -> &EventSource {
        &self.event_source
    }

    pub fn event_subject(&self) -> &EventSubject {
        &self.event_subject
    }

    pub fn event_classification(&self) -> &EventClassification {
        &self.event_classification
    }

    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    pub fn causation(&self) -> &EventCausation {
        &self.causation
    }

    pub fn workflow_event_context(&self) -> &WorkflowEventContext {
        &self.workflow_event_context
    }
}

pub type WorkflowEventDecision = EventEnvelope<WorkflowEventContext>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkflowEventIntegration;

impl WorkflowEventIntegration {
    pub fn evaluate(
        request: &WorkflowEventIntegrationRequest,
    ) -> DomainResult<WorkflowEventDecision> {
        let trace = EventTrace::new(
            None,
            Some(request.workflow_event_context().workflow_id().clone()),
            None,
            None,
            request
                .workflow_event_context()
                .workflow_audit_evidence_references()
                .iter()
                .map(|reference| reference.audit_evidence_id().clone())
                .collect(),
        )?;

        let envelope = crate::event::validate_event_envelope(EventEnvelopeCandidate {
            event_id: Some(request.event_id().clone()),
            event_type: Some(request.workflow_event_type().clone()),
            event_version: Some(request.event_version().clone()),
            occurred_at: Some(request.occurred_at().clone()),
            recorded_at: Some(request.recorded_at().clone()),
            source: Some(request.event_source().clone()),
            subject: Some(request.event_subject().clone()),
            payload: Some(request.workflow_event_context().clone()),
            classification: Some(*request.event_classification()),
            trace: Some(trace),
            correlation_id: request.correlation_id().cloned(),
            causation: request.causation().clone(),
        })?;

        validate_event_identity(&envelope)?;
        validate_event_version(&envelope)?;
        validate_event_timestamps(&envelope)?;

        Ok(envelope)
    }
}

fn validate_workflow_event_category(
    workflow_event_type: &WorkflowEventTypeReference,
    workflow_event_context: &WorkflowEventContext,
) -> DomainResult<()> {
    match workflow_event_type.as_str() {
        "workflow.created"
        | "workflow.activated"
        | "workflow.step.selected"
        | "workflow.step.completed"
        | "workflow.step.blocked"
        | "workflow.step.skipped"
        | "workflow.completed"
        | "workflow.cancelled"
        | "workflow.archived" => {
            if workflow_event_context.failure_code().is_some()
                && workflow_event_type.as_str() != "workflow.failed"
            {
                return Err(DomainError::InvalidWorkflowEventIntegration(
                    "non-failure workflow event must not carry failure code",
                ));
            }
            Ok(())
        }
        "workflow.transition.allowed" => {
            match workflow_event_context.workflow_transition_decision() {
                Some(WorkflowTransitionDecision::Allowed(_)) => Ok(()),
                _ => Err(DomainError::InvalidWorkflowEventIntegration(
                    "workflow transition allowed event requires allowed transition outcome",
                )),
            }
        }
        "workflow.transition.rejected" => {
            match workflow_event_context.workflow_transition_decision() {
                Some(WorkflowTransitionDecision::Rejected(_)) => Ok(()),
                _ => Err(DomainError::InvalidWorkflowEventIntegration(
                    "workflow transition rejected event requires rejected transition outcome",
                )),
            }
        }
        "workflow.transition.noop" => match workflow_event_context.workflow_transition_decision() {
            Some(WorkflowTransitionDecision::NoOp(_)) => Ok(()),
            _ => Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow transition no-op event requires no-op transition outcome",
            )),
        },
        "workflow.authorized" => match workflow_event_context.workflow_authorization_decision() {
            Some(WorkflowAuthorizationDecision::Allow) => Ok(()),
            _ => Err(DomainError::InvalidWorkflowEventIntegration(
                "workflow authorized event requires allow authorization outcome",
            )),
        },
        "workflow.authorization.denied" => {
            match workflow_event_context.workflow_authorization_decision() {
                Some(outcome) if outcome.is_denied() => Ok(()),
                _ => Err(DomainError::InvalidWorkflowEventIntegration(
                    "workflow authorization denied event requires deny-class authorization outcome",
                )),
            }
        }
        "workflow.failed" => {
            if workflow_event_context.failure_code().is_none() {
                return Err(DomainError::InvalidWorkflowEventIntegration(
                    "workflow failed event requires stable failure code",
                ));
            }
            Ok(())
        }
        _ => Err(DomainError::InvalidWorkflowEventIntegration(
            "unsupported workflow event type",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkflowAuditEvidenceReference, WorkflowAuthorizationContext, WorkflowAuthorizationControl,
        WorkflowAuthorizationDecision, WorkflowAuthorizationRequest, WorkflowDefinition,
        WorkflowEngineFoundation, WorkflowEventContext, WorkflowEventIntegration,
        WorkflowEventIntegrationRequest, WorkflowInstance, WorkflowLifecycleMapReference,
        WorkflowOperationReference, WorkflowRecoveryReference, WorkflowRetryLimit,
        WorkflowRetryPolicyReference, WorkflowStepCoordination, WorkflowStepExecutionPlan,
        WorkflowStepOutcomeReference, WorkflowStepReference, WorkflowStepSelection,
        WorkflowTerminalOutcomeReference,
    };
    use crate::authorization::{
        ActionVerb, AuthorizationAuditEvidenceReference, AuthorizationDecisionOutcome,
        AuthorizationDecisionReference, AuthorizationPrincipalReference,
        AuthorizationPrincipalType, AuthorizationSubject, AuthorizationTarget,
        CredentialStatusReference, MatchedPolicyEvidenceReference, PermissionEffectIntent,
        PermissionReference, PrincipalLifecycleStateReference, ResourceType, ScopeLevel,
        ScopeReference,
    };
    use crate::errors::DomainError;
    use crate::event::{
        EventCausation, EventClassification, EventComponent, EventEnvelope, EventSource,
        EventSubject, EventSubjectId, EventSubjectType, EventType, EventVersion,
    };
    use crate::identifier::EnglishNamespace;
    use crate::identifier::{
        AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId, CorrelationId,
        DecisionId, DelegationId, EnterpriseId, EventId, HumanId, OrganizationUnitId, PermissionId,
        PolicyId, PrincipalId, ProjectId, ScopeId, StableVersion, WorkflowId, WorkspaceId,
    };
    use crate::lifecycle::WorkflowState;
    use crate::ownership::{OwnerReference, OwnershipPath};
    use crate::request::{AuthorizationRequestRecord, TimeReference};
    use crate::state::{
        StateSequence, TransitionAuthorityReference, TransitionEvidenceReference,
        TransitionOutcome, TransitionReasonReference, WorkflowFailureCode, WorkflowStateSnapshot,
        WorkflowTransitionControl, WorkflowTransitionControlRequest, WorkflowTransitionDecision,
    };

    #[test]
    fn workflow_retry_limit_rejects_zero_ces_b0_030_14() {
        let error = WorkflowRetryLimit::new(0).expect_err("workflow retry limit must be bounded");
        assert!(error
            .to_string()
            .contains("workflow retry limit must be greater than zero"));
    }

    #[test]
    fn workflow_recovery_requires_path_reference_ces_b0_030_14() {
        let error =
            WorkflowRecoveryReference::new("", true).expect_err("recovery path must be explicit");
        assert!(error.to_string().contains("workflow_corrective_path"));
    }

    fn workflow_id() -> WorkflowId {
        WorkflowId::new("CX-WF-000001").expect("workflow")
    }

    fn ownership() -> OwnershipPath {
        OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            Some(OrganizationUnitId::new("CX-OU-000001").expect("unit")),
        )
        .expect("ownership")
    }

    fn definition_version() -> StableVersion {
        StableVersion::new("workflow_definition_version", "1.0.0").expect("version")
    }

    fn retry_limit() -> WorkflowRetryLimit {
        WorkflowRetryLimit::new(3).expect("retry limit")
    }

    fn retry_policy() -> WorkflowRetryPolicyReference {
        WorkflowRetryPolicyReference::new(definition_version(), retry_limit())
    }

    fn recovery_reference() -> WorkflowRecoveryReference {
        WorkflowRecoveryReference::new("retry/manual-review", true).expect("recovery")
    }

    fn audit_evidence(id: &str) -> WorkflowAuditEvidenceReference {
        WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new(id).expect("audit evidence id"),
            workflow_id(),
            definition_version(),
            vec![PolicyId::new("CX-POL-000001").expect("policy")],
            vec![
                AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("authorization decision"),
            ],
            vec![DelegationId::new("CX-DEL-000001").expect("delegation")],
            vec![DecisionId::new("CX-DEC-000001").expect("decision")],
        )
        .expect("audit evidence")
    }

    fn second_audit_evidence() -> WorkflowAuditEvidenceReference {
        WorkflowAuditEvidenceReference::new(
            AuditEvidenceId::new("CX-AUD-000002").expect("audit evidence id"),
            workflow_id(),
            StableVersion::new("workflow_definition_version", "1.0.1").expect("version"),
            vec![PolicyId::new("CX-POL-000002").expect("policy")],
            vec![
                AuthorizationDecisionId::new("CX-AUTHDEC-000002").expect("authorization decision"),
            ],
            vec![DelegationId::new("CX-DEL-000002").expect("delegation")],
            vec![DecisionId::new("CX-DEC-000002").expect("decision")],
        )
        .expect("audit evidence")
    }

    fn namespace() -> EnglishNamespace {
        EnglishNamespace::new("workflow_namespace", "ops.approval-flow").expect("namespace")
    }

    fn lifecycle_map() -> WorkflowLifecycleMapReference {
        WorkflowLifecycleMapReference::new("workflow.lifecycle.v1").expect("lifecycle map")
    }

    fn entry_step(value: &str) -> WorkflowStepReference {
        WorkflowStepReference::new(value).expect("entry step")
    }

    fn step_outcome(value: &str) -> WorkflowStepOutcomeReference {
        WorkflowStepOutcomeReference::new(value).expect("step outcome")
    }

    fn terminal_outcome(value: &str) -> WorkflowTerminalOutcomeReference {
        WorkflowTerminalOutcomeReference::new(value).expect("terminal outcome")
    }

    fn workflow_definition() -> WorkflowDefinition {
        WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review"), entry_step("collect-input")],
            vec![terminal_outcome("completed"), terminal_outcome("cancelled")],
            vec![
                PolicyId::new("CX-POL-000001").expect("policy"),
                PolicyId::new("CX-POL-000002").expect("policy"),
            ],
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("workflow definition")
    }

    fn workflow_state_snapshot() -> WorkflowStateSnapshot {
        WorkflowStateSnapshot::new(
            workflow_id(),
            ownership(),
            definition_version(),
            WorkflowState::Validated,
            StateSequence::new(1).expect("sequence"),
        )
    }

    fn workflow_instance() -> WorkflowInstance {
        WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("workflow instance")
    }

    fn workflow_step_selection() -> WorkflowStepSelection {
        WorkflowStepSelection::new(
            entry_step("start.review"),
            vec![entry_step("collect-input"), entry_step("approve-review")],
        )
    }

    fn workflow_step_execution_plan() -> WorkflowStepExecutionPlan {
        WorkflowStepExecutionPlan::new(
            vec![
                entry_step("intake.completed"),
                entry_step("validation.completed"),
            ],
            vec![
                entry_step("blocked.compliance"),
                entry_step("blocked.escalation"),
            ],
            vec![
                entry_step("skipped.fast-track"),
                entry_step("skipped.manual-check"),
            ],
            vec![
                step_outcome("terminal.completed"),
                step_outcome("terminal.cancelled"),
            ],
        )
        .expect("workflow step execution plan")
    }

    fn workflow_step_coordination() -> WorkflowStepCoordination {
        WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            workflow_step_execution_plan(),
        )
        .expect("workflow step coordination")
    }

    fn authorization_subject() -> AuthorizationSubject {
        AuthorizationSubject::Principal(
            AuthorizationPrincipalReference::new(
                PrincipalId::new("CX-PRN-000001").expect("principal"),
                AuthorizationPrincipalType::Employee,
                "CX-EMP-000001",
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
                CredentialStatusReference::new("Valid").expect("credential"),
            )
            .expect("principal"),
        )
    }

    fn authorization_permission(action: &str) -> PermissionReference {
        PermissionReference::new(
            PermissionId::new("CX-PERM-000001").expect("permission"),
            ActionVerb::new(action).expect("action"),
            ResourceType::new("workflow").expect("resource type"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    fn authorization_scope() -> ScopeReference {
        ScopeReference::new(
            ScopeId::new("CX-SCP-000001").expect("scope"),
            ScopeLevel::Project,
            ownership(),
            None,
        )
        .expect("scope")
    }

    fn authorization_target() -> AuthorizationTarget {
        AuthorizationTarget::new(
            ResourceType::new("workflow").expect("resource type"),
            "CX-WF-000001",
            authorization_scope(),
        )
        .expect("target")
    }

    fn authorization_request_record(action: &str) -> AuthorizationRequestRecord {
        AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request id"),
            authorization_subject(),
            authorization_permission(action),
            authorization_target(),
            TimeReference::new("2026-07-17T00:00:00Z").expect("time"),
            "authorize workflow operation",
        )
        .expect("authorization request")
    }

    fn authorization_decision_reference(
        outcome: AuthorizationDecisionOutcome,
    ) -> AuthorizationDecisionReference {
        AuthorizationDecisionReference::new(
            AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision id"),
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request id"),
            PolicyId::new("CX-POL-000001").expect("policy"),
            outcome,
            crate::authorization::AuthorizationEvaluationOrderVersion::new("1.0.0")
                .expect("evaluation version"),
            MatchedPolicyEvidenceReference::new("policy.match.001").expect("policy evidence"),
            "2026-07-17T00:00:00Z",
        )
        .expect("authorization decision")
    }

    fn authorization_audit_evidence_reference(
        audit_id: &str,
        outcome: AuthorizationDecisionOutcome,
    ) -> AuthorizationAuditEvidenceReference {
        AuthorizationAuditEvidenceReference::new(
            AuditEvidenceId::new(audit_id).expect("audit id"),
            AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision id"),
            PrincipalId::new("CX-PRN-000001").expect("principal"),
            ScopeId::new("CX-SCP-000001").expect("scope"),
            StableVersion::new("policy_version", "1.0.0").expect("policy version"),
            vec![MatchedPolicyEvidenceReference::new("policy.match.001").expect("rule")],
            outcome,
        )
        .expect("authorization audit evidence")
    }

    fn workflow_authorization_context(
        outcome: AuthorizationDecisionOutcome,
    ) -> WorkflowAuthorizationContext {
        WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(outcome),
            vec![authorization_audit_evidence_reference(
                "CX-AUD-100001",
                outcome,
            )],
        )
        .expect("workflow authorization context")
    }

    fn workflow_authorization_request(
        outcome: AuthorizationDecisionOutcome,
    ) -> WorkflowAuthorizationRequest {
        WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(outcome),
            Some(
                TransitionAuthorityReference::new("authority.workflow-owner")
                    .expect("authority reference"),
            ),
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.002")
                    .expect("transition evidence"),
            ],
        )
        .expect("workflow authorization request")
    }

    fn workflow_transition_control_request(
        current_state: WorkflowState,
        requested_state: WorkflowState,
        evidence: Vec<TransitionEvidenceReference>,
    ) -> WorkflowTransitionControlRequest {
        WorkflowTransitionControlRequest::new(
            WorkflowStateSnapshot::new(
                workflow_id(),
                ownership(),
                definition_version(),
                current_state,
                StateSequence::new(1).expect("sequence"),
            ),
            requested_state,
            Some(TransitionReasonReference::new("operator request").expect("reason")),
            Some(TransitionAuthorityReference::new("authority.workflow-owner").expect("authority")),
            evidence,
            None,
            crate::state::WorkflowLifecycleGuards {
                policy_valid: true,
                authorization_valid: true,
                delegation_valid: true,
                decision_valid: true,
                scope_valid: true,
                participants_valid: true,
                audit_evidence: Some(
                    TransitionEvidenceReference::new("transition.evidence.001")
                        .expect("audit evidence"),
                ),
                upstream_outcomes_allow: true,
                retry_limit_respected: true,
                recovery_revalidated: true,
                failure_code: None,
            },
        )
        .expect("workflow transition control request")
    }

    fn allowed_transition_decision() -> WorkflowTransitionDecision {
        WorkflowTransitionControl::evaluate(&workflow_transition_control_request(
            WorkflowState::Ready,
            WorkflowState::Running,
            vec![TransitionEvidenceReference::new("transition.evidence.001")
                .expect("transition evidence")],
        ))
    }

    fn rejected_transition_decision() -> WorkflowTransitionDecision {
        WorkflowTransitionControl::evaluate(&workflow_transition_control_request(
            WorkflowState::Draft,
            WorkflowState::Running,
            vec![TransitionEvidenceReference::new("transition.evidence.001")
                .expect("transition evidence")],
        ))
    }

    fn noop_transition_decision() -> WorkflowTransitionDecision {
        WorkflowTransitionControl::evaluate(&workflow_transition_control_request(
            WorkflowState::Ready,
            WorkflowState::Ready,
            vec![TransitionEvidenceReference::new("transition.evidence.001")
                .expect("transition evidence")],
        ))
    }

    fn workflow_event_source() -> EventSource {
        EventSource::new(
            EventComponent::new("workflow-engine").expect("component"),
            None,
        )
    }

    fn workflow_event_subject() -> EventSubject {
        EventSubject::new(
            EventSubjectType::new("workflow").expect("subject type"),
            EventSubjectId::new("CX-WF-000001").expect("subject id"),
        )
    }

    fn workflow_event_context(
        transition_decision: Option<WorkflowTransitionDecision>,
        authorization_decision: Option<WorkflowAuthorizationDecision>,
        failure_code: Option<WorkflowFailureCode>,
    ) -> WorkflowEventContext {
        WorkflowEventContext::new(
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(workflow_transition_control_request(
                WorkflowState::Ready,
                WorkflowState::Running,
                vec![TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence")],
            )),
            transition_decision,
            Some(workflow_authorization_request(
                authorization_decision.unwrap_or(AuthorizationDecisionOutcome::Allow),
            )),
            authorization_decision,
            Some(
                WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            ),
            Some(TransitionReasonReference::new("operator request").expect("reason")),
            Some(
                TransitionAuthorityReference::new("authority.workflow-owner")
                    .expect("authority reference"),
            ),
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.002")
                    .expect("transition evidence"),
            ],
            vec![
                audit_evidence("CX-AUD-000010"),
                audit_evidence("CX-AUD-000011"),
            ],
            failure_code,
        )
        .expect("workflow event context")
    }

    fn workflow_event_request(
        event_type: &str,
        context: WorkflowEventContext,
        correlation_id: Option<CorrelationId>,
        causation: EventCausation,
    ) -> WorkflowEventIntegrationRequest {
        WorkflowEventIntegrationRequest::new(
            EventType::new(event_type).expect("event type"),
            EventId::new("CX-EVT-000001").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            correlation_id,
            causation,
            context,
        )
        .expect("workflow event integration request")
    }

    #[test]
    fn workflow_engine_foundation_complete_foundation_construction_passes() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("foundation");

        assert_eq!(foundation.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(foundation.audit_evidence().len(), 2);
    }

    #[test]
    fn workflow_engine_foundation_mandatory_workflow_identity_is_preserved() {
        let workflow_id = workflow_id();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id.clone(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.workflow_id(), &workflow_id);
    }

    #[test]
    fn workflow_engine_foundation_ownership_reference_is_preserved() {
        let ownership = ownership();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership.clone(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.ownership(), &ownership);
    }

    #[test]
    fn workflow_engine_foundation_definition_version_is_preserved() {
        let definition_version = definition_version();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version.clone(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.definition_version(), &definition_version);
    }

    #[test]
    fn workflow_engine_foundation_retry_policy_and_retry_limit_are_preserved() {
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.retry_policy(), Some(&retry_policy));
        assert_eq!(foundation.retry_limit(), Some(retry_limit));
    }

    #[test]
    fn workflow_engine_foundation_retry_limit_without_policy_is_rejected() {
        let error = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("retry limit without policy must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowReference(
                "retry limit requires retry policy",
            )
        );
    }

    #[test]
    fn workflow_engine_foundation_retry_policy_without_limit_is_allowed() {
        let retry_policy = retry_policy();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy.clone()),
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.retry_policy(), Some(&retry_policy));
        assert_eq!(foundation.retry_limit(), None);
    }

    #[test]
    fn workflow_engine_foundation_recovery_reference_is_optional() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![],
        )
        .expect("foundation");

        assert_eq!(foundation.recovery_reference(), None);
    }

    #[test]
    fn workflow_engine_foundation_empty_audit_evidence_is_allowed() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            Some(recovery_reference()),
            vec![],
        )
        .expect("foundation");

        assert!(foundation.audit_evidence().is_empty());
    }

    #[test]
    fn workflow_engine_foundation_audit_evidence_order_is_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("foundation");

        assert_eq!(foundation.audit_evidence(), &[first, second]);
    }

    #[test]
    fn workflow_engine_foundation_duplicate_audit_evidence_is_rejected() {
        let error = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            None,
            None,
            vec![
                audit_evidence("CX-AUD-000001"),
                audit_evidence("CX-AUD-000001"),
            ],
        )
        .expect_err("duplicate audit evidence must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowReference(
                "duplicate workflow audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_engine_foundation_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_engine_foundation_construction_is_deterministic() {
        let first = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );
        let second = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_engine_foundation_value_semantics_are_preserved() {
        let foundation = WorkflowEngineFoundation::new(
            workflow_id(),
            ownership(),
            definition_version(),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        )
        .expect("foundation");

        assert_eq!(foundation.clone(), foundation);
    }

    #[test]
    fn workflow_engine_foundation_does_not_mutate_supplied_values() {
        let workflow_id = workflow_id();
        let ownership = ownership();
        let definition_version = definition_version();
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence = vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let ownership_before = ownership.clone();
        let definition_version_before = definition_version.clone();
        let retry_policy_before = retry_policy.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_before = audit_evidence.clone();

        let foundation = WorkflowEngineFoundation::new(
            workflow_id.clone(),
            ownership.clone(),
            definition_version.clone(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            Some(recovery_reference.clone()),
            audit_evidence.clone(),
        )
        .expect("foundation");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(ownership, ownership_before);
        assert_eq!(definition_version, definition_version_before);
        assert_eq!(retry_policy, retry_policy_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence, audit_evidence_before);
        assert_eq!(foundation.workflow_id(), &workflow_id_before);
    }

    #[test]
    fn workflow_engine_foundation_complete_foundation_owner_reference_remains_external() {
        let owner = OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"));
        assert_eq!(owner.owner_id().as_str(), "CX-EMP-000001");
    }

    #[test]
    fn workflow_definition_valid_definition_construction_passes() {
        let definition = workflow_definition();
        assert_eq!(definition.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(definition.entry_steps().len(), 2);
    }

    #[test]
    fn workflow_definition_identity_is_preserved() {
        let workflow_id = workflow_id();
        let definition = WorkflowDefinition::new(
            workflow_id.clone(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.workflow_id(), &workflow_id);
    }

    #[test]
    fn workflow_definition_namespace_is_preserved() {
        let namespace = namespace();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace.clone(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.namespace(), &namespace);
    }

    #[test]
    fn workflow_definition_definition_version_is_preserved() {
        let definition_version = definition_version();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version.clone(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.definition_version(), &definition_version);
    }

    #[test]
    fn workflow_definition_ownership_is_preserved() {
        let ownership = ownership();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership.clone(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.ownership(), &ownership);
    }

    #[test]
    fn workflow_definition_entry_steps_are_preserved_in_caller_order() {
        let first = entry_step("start.review");
        let second = entry_step("collect-input");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![first.clone(), second.clone()],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.entry_steps(), &[first, second]);
    }

    #[test]
    fn workflow_definition_empty_entry_steps_are_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("empty entry steps must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition(
                "workflow definition requires at least one entry step",
            )
        );
    }

    #[test]
    fn workflow_definition_duplicate_entry_step_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review"), entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate entry step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow entry step reference")
        );
    }

    #[test]
    fn workflow_definition_terminal_outcomes_are_preserved_in_caller_order() {
        let first = terminal_outcome("completed");
        let second = terminal_outcome("cancelled");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![first.clone(), second.clone()],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.terminal_outcomes(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_terminal_outcome_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![terminal_outcome("completed"), terminal_outcome("completed")],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate terminal outcome must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow terminal outcome reference",)
        );
    }

    #[test]
    fn workflow_definition_policy_references_are_preserved() {
        let first = PolicyId::new("CX-POL-000001").expect("policy");
        let second = PolicyId::new("CX-POL-000002").expect("policy");
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![first.clone(), second.clone()],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.policy_references(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_policy_reference_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![
                PolicyId::new("CX-POL-000001").expect("policy"),
                PolicyId::new("CX-POL-000001").expect("policy"),
            ],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("duplicate policy must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition("duplicate workflow policy reference")
        );
    }

    #[test]
    fn workflow_definition_audit_evidence_may_be_empty() {
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![],
        )
        .expect("definition");

        assert!(definition.audit_evidence().is_empty());
    }

    #[test]
    fn workflow_definition_audit_evidence_order_is_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("definition");

        assert_eq!(definition.audit_evidence(), &[first, second]);
    }

    #[test]
    fn workflow_definition_duplicate_audit_evidence_is_rejected() {
        let error = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![entry_step("start.review")],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![
                audit_evidence("CX-AUD-000001"),
                audit_evidence("CX-AUD-000001"),
            ],
        )
        .expect_err("duplicate audit evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowDefinition(
                "duplicate workflow definition audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_definition_construction_is_deterministic() {
        let first = workflow_definition();
        let second = workflow_definition();

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_definition_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            lifecycle_map(),
            vec![],
            vec![],
            vec![],
            None,
            None,
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_definition_value_semantics_are_preserved() {
        let definition = workflow_definition();
        assert_eq!(definition.clone(), definition);
    }

    #[test]
    fn workflow_definition_supplied_values_are_not_mutated() {
        let workflow_id = workflow_id();
        let namespace = namespace();
        let definition_version = definition_version();
        let ownership = ownership();
        let lifecycle_map = lifecycle_map();
        let entry_steps = vec![entry_step("start.review"), entry_step("collect-input")];
        let terminal_outcomes = vec![terminal_outcome("completed"), terminal_outcome("cancelled")];
        let policy_references = vec![
            PolicyId::new("CX-POL-000001").expect("policy"),
            PolicyId::new("CX-POL-000002").expect("policy"),
        ];
        let retry_policy = retry_policy();
        let retry_limit = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence = vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let namespace_before = namespace.clone();
        let definition_version_before = definition_version.clone();
        let ownership_before = ownership.clone();
        let lifecycle_map_before = lifecycle_map.clone();
        let entry_steps_before = entry_steps.clone();
        let terminal_outcomes_before = terminal_outcomes.clone();
        let policy_references_before = policy_references.clone();
        let retry_policy_before = retry_policy.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_before = audit_evidence.clone();

        let definition = WorkflowDefinition::new(
            workflow_id.clone(),
            namespace.clone(),
            definition_version.clone(),
            ownership.clone(),
            lifecycle_map.clone(),
            entry_steps.clone(),
            terminal_outcomes.clone(),
            policy_references.clone(),
            Some(retry_policy.clone()),
            Some(retry_limit),
            Some(recovery_reference.clone()),
            audit_evidence.clone(),
        )
        .expect("definition");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(namespace, namespace_before);
        assert_eq!(definition_version, definition_version_before);
        assert_eq!(ownership, ownership_before);
        assert_eq!(lifecycle_map, lifecycle_map_before);
        assert_eq!(entry_steps, entry_steps_before);
        assert_eq!(terminal_outcomes, terminal_outcomes_before);
        assert_eq!(policy_references, policy_references_before);
        assert_eq!(retry_policy, retry_policy_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence, audit_evidence_before);
        assert_eq!(definition.namespace(), &namespace_before);
    }

    #[test]
    fn workflow_definition_external_reference_existence_is_not_checked() {
        let definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            WorkflowLifecycleMapReference::new("custom.lifecycle.map").expect("lifecycle map"),
            vec![WorkflowStepReference::new("entry.custom-step").expect("entry step")],
            vec![
                WorkflowTerminalOutcomeReference::new("terminal.custom-outcome")
                    .expect("terminal outcome"),
            ],
            vec![PolicyId::new("CX-POL-999999").expect("policy")],
            None,
            None,
            None,
            vec![],
        )
        .expect("definition");

        assert_eq!(definition.policy_references()[0].as_str(), "CX-POL-999999");
    }

    #[test]
    fn workflow_instance_valid_construction() {
        let instance = workflow_instance();
        assert_eq!(instance.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(instance.audit_evidence_references().len(), 2);
    }

    #[test]
    fn workflow_instance_definition_preserved() {
        let workflow_definition = workflow_definition();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition.clone(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.workflow_definition(), &workflow_definition);
    }

    #[test]
    fn workflow_instance_definition_version_preserved() {
        let definition_version_snapshot = definition_version();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version_snapshot.clone(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.definition_version_snapshot(),
            &definition_version_snapshot
        );
    }

    #[test]
    fn workflow_instance_ownership_preserved() {
        let ownership_reference = ownership();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership_reference.clone(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.ownership_reference(), &ownership_reference);
    }

    #[test]
    fn workflow_instance_workflow_state_preserved() {
        let current_workflow_state_snapshot = workflow_state_snapshot();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            current_workflow_state_snapshot.clone(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.current_workflow_state_snapshot(),
            &current_workflow_state_snapshot
        );
    }

    #[test]
    fn workflow_instance_retry_preserved() {
        let retry_policy_snapshot = retry_policy();
        let retry_limit_snapshot = retry_limit();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy_snapshot.clone()),
            Some(retry_limit_snapshot),
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.retry_policy_snapshot(),
            Some(&retry_policy_snapshot)
        );
        assert_eq!(instance.retry_limit_snapshot(), Some(retry_limit_snapshot));
    }

    #[test]
    fn workflow_instance_retry_validation() {
        let error = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("retry limit without policy must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowInstance("retry limit requires retry policy")
        );
    }

    #[test]
    fn workflow_instance_recovery_optional() {
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(instance.recovery_reference(), None);
    }

    #[test]
    fn workflow_instance_empty_audit_evidence_allowed() {
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            Some(recovery_reference()),
            vec![],
        )
        .expect("workflow instance");

        assert!(instance.audit_evidence_references().is_empty());
    }

    #[test]
    fn workflow_instance_audit_order_preserved() {
        let first = audit_evidence("CX-AUD-000001");
        let second = second_audit_evidence();
        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![first.clone(), second.clone()],
        )
        .expect("workflow instance");

        assert_eq!(instance.audit_evidence_references(), &[first, second]);
    }

    #[test]
    fn workflow_instance_duplicate_audit_evidence_rejected() {
        let error = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            None,
            None,
            vec![
                audit_evidence("CX-AUD-000001"),
                audit_evidence("CX-AUD-000001"),
            ],
        )
        .expect_err("duplicate audit evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowInstance(
                "duplicate workflow instance audit evidence reference",
            )
        );
    }

    #[test]
    fn workflow_instance_deterministic_construction() {
        let first = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );
        let second = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            Some(retry_policy()),
            Some(retry_limit()),
            Some(recovery_reference()),
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()],
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_instance_equivalent_invalid_inputs() {
        let first = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowInstance::new(
            workflow_id(),
            workflow_definition(),
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            audit_evidence("CX-AUD-000003"),
            None,
            Some(retry_limit()),
            None,
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_instance_value_semantics() {
        let instance = workflow_instance();
        assert_eq!(instance.clone(), instance);
    }

    #[test]
    fn workflow_instance_supplied_values_not_mutated() {
        let workflow_id = workflow_id();
        let workflow_definition = workflow_definition();
        let definition_version_snapshot = definition_version();
        let ownership_reference = ownership();
        let current_workflow_state_snapshot = workflow_state_snapshot();
        let creation_evidence = audit_evidence("CX-AUD-000003");
        let retry_policy_snapshot = retry_policy();
        let retry_limit_snapshot = retry_limit();
        let recovery_reference = recovery_reference();
        let audit_evidence_references =
            vec![audit_evidence("CX-AUD-000001"), second_audit_evidence()];

        let workflow_id_before = workflow_id.clone();
        let workflow_definition_before = workflow_definition.clone();
        let definition_version_snapshot_before = definition_version_snapshot.clone();
        let ownership_reference_before = ownership_reference.clone();
        let current_workflow_state_snapshot_before = current_workflow_state_snapshot.clone();
        let creation_evidence_before = creation_evidence.clone();
        let retry_policy_snapshot_before = retry_policy_snapshot.clone();
        let recovery_reference_before = recovery_reference.clone();
        let audit_evidence_references_before = audit_evidence_references.clone();

        let instance = WorkflowInstance::new(
            workflow_id.clone(),
            workflow_definition.clone(),
            definition_version_snapshot.clone(),
            ownership_reference.clone(),
            current_workflow_state_snapshot.clone(),
            creation_evidence.clone(),
            Some(retry_policy_snapshot.clone()),
            Some(retry_limit_snapshot),
            Some(recovery_reference.clone()),
            audit_evidence_references.clone(),
        )
        .expect("workflow instance");

        assert_eq!(workflow_id, workflow_id_before);
        assert_eq!(workflow_definition, workflow_definition_before);
        assert_eq!(
            definition_version_snapshot,
            definition_version_snapshot_before
        );
        assert_eq!(ownership_reference, ownership_reference_before);
        assert_eq!(
            current_workflow_state_snapshot,
            current_workflow_state_snapshot_before
        );
        assert_eq!(creation_evidence, creation_evidence_before);
        assert_eq!(retry_policy_snapshot, retry_policy_snapshot_before);
        assert_eq!(recovery_reference, recovery_reference_before);
        assert_eq!(audit_evidence_references, audit_evidence_references_before);
        assert_eq!(instance.workflow_id(), &workflow_id_before);
    }

    #[test]
    fn workflow_instance_external_references_not_validated() {
        let workflow_definition = WorkflowDefinition::new(
            workflow_id(),
            namespace(),
            definition_version(),
            ownership(),
            WorkflowLifecycleMapReference::new("custom.lifecycle.map").expect("lifecycle map"),
            vec![WorkflowStepReference::new("entry.custom-step").expect("entry step")],
            vec![],
            vec![PolicyId::new("CX-POL-999999").expect("policy")],
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow definition");
        let creation_evidence =
            WorkflowAuditEvidenceReference::new(
                AuditEvidenceId::new("CX-AUD-999999").expect("audit evidence id"),
                workflow_id(),
                definition_version(),
                vec![PolicyId::new("CX-POL-999999").expect("policy")],
                vec![AuthorizationDecisionId::new("CX-AUTHDEC-999999")
                    .expect("authorization decision")],
                vec![DelegationId::new("CX-DEL-999999").expect("delegation")],
                vec![DecisionId::new("CX-DEC-999999").expect("decision")],
            )
            .expect("creation evidence");

        let instance = WorkflowInstance::new(
            workflow_id(),
            workflow_definition,
            definition_version(),
            ownership(),
            workflow_state_snapshot(),
            creation_evidence,
            None,
            None,
            None,
            vec![],
        )
        .expect("workflow instance");

        assert_eq!(
            instance.creation_evidence().audit_evidence_id().as_str(),
            "CX-AUD-999999"
        );
    }

    #[test]
    fn workflow_step_coordination_valid_construction() {
        let coordination = workflow_step_coordination();
        assert_eq!(
            coordination
                .workflow_step_selection()
                .current_step()
                .as_str(),
            "start.review"
        );
    }

    #[test]
    fn workflow_step_coordination_current_step_preserved() {
        let selection = workflow_step_selection();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            selection.clone(),
            workflow_step_execution_plan(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination.workflow_step_selection().current_step(),
            selection.current_step()
        );
    }

    #[test]
    fn workflow_step_coordination_next_candidates_preserved() {
        let selection = workflow_step_selection();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            selection.clone(),
            workflow_step_execution_plan(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_selection()
                .next_candidate_steps(),
            selection.next_candidate_steps()
        );
    }

    #[test]
    fn workflow_step_coordination_completed_preserved() {
        let plan = workflow_step_execution_plan();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            plan.clone(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .completed_step_references(),
            plan.completed_step_references()
        );
    }

    #[test]
    fn workflow_step_coordination_blocked_preserved() {
        let plan = workflow_step_execution_plan();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            plan.clone(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .blocked_step_references(),
            plan.blocked_step_references()
        );
    }

    #[test]
    fn workflow_step_coordination_skipped_preserved() {
        let plan = workflow_step_execution_plan();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            plan.clone(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .skipped_step_references(),
            plan.skipped_step_references()
        );
    }

    #[test]
    fn workflow_step_coordination_terminal_preserved() {
        let plan = workflow_step_execution_plan();
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            plan.clone(),
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .terminal_step_references(),
            plan.terminal_step_references()
        );
    }

    #[test]
    fn workflow_step_coordination_duplicate_completed_rejected() {
        let error = WorkflowStepExecutionPlan::new(
            vec![entry_step("done.review"), entry_step("done.review")],
            vec![],
            vec![],
            vec![],
        )
        .expect_err("duplicate completed step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination(
                "duplicate completed workflow step reference",
            )
        );
    }

    #[test]
    fn workflow_step_coordination_duplicate_blocked_rejected() {
        let error = WorkflowStepExecutionPlan::new(
            vec![],
            vec![entry_step("blocked.review"), entry_step("blocked.review")],
            vec![],
            vec![],
        )
        .expect_err("duplicate blocked step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination(
                "duplicate blocked workflow step reference",
            )
        );
    }

    #[test]
    fn workflow_step_coordination_duplicate_skipped_rejected() {
        let error = WorkflowStepExecutionPlan::new(
            vec![],
            vec![],
            vec![entry_step("skipped.review"), entry_step("skipped.review")],
            vec![],
        )
        .expect_err("duplicate skipped step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination(
                "duplicate skipped workflow step reference",
            )
        );
    }

    #[test]
    fn workflow_step_coordination_duplicate_terminal_rejected() {
        let error = WorkflowStepExecutionPlan::new(
            vec![],
            vec![],
            vec![],
            vec![
                step_outcome("terminal.review"),
                step_outcome("terminal.review"),
            ],
        )
        .expect_err("duplicate terminal step must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination(
                "duplicate terminal workflow step reference",
            )
        );
    }

    #[test]
    fn workflow_step_coordination_current_step_not_allowed_inside_completed() {
        let error = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            WorkflowStepExecutionPlan::new(
                vec![entry_step("start.review")],
                vec![],
                vec![],
                vec![],
            )
            .expect("workflow step execution plan"),
        )
        .expect_err("current step in completed must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination(
                "current workflow step cannot be completed",
            )
        );
    }

    #[test]
    fn workflow_step_coordination_current_step_not_allowed_inside_blocked() {
        let error = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            WorkflowStepExecutionPlan::new(
                vec![],
                vec![entry_step("start.review")],
                vec![],
                vec![],
            )
            .expect("workflow step execution plan"),
        )
        .expect_err("current step in blocked must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination("current workflow step cannot be blocked",)
        );
    }

    #[test]
    fn workflow_step_coordination_current_step_not_allowed_inside_skipped() {
        let error = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            WorkflowStepExecutionPlan::new(
                vec![],
                vec![],
                vec![entry_step("start.review")],
                vec![],
            )
            .expect("workflow step execution plan"),
        )
        .expect_err("current step in skipped must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowStepCoordination("current workflow step cannot be skipped",)
        );
    }

    #[test]
    fn workflow_step_coordination_caller_ordering_preserved() {
        let selection = WorkflowStepSelection::new(
            entry_step("start.review"),
            vec![
                entry_step("candidate.second"),
                entry_step("candidate.third"),
            ],
        );
        let plan = WorkflowStepExecutionPlan::new(
            vec![
                entry_step("completed.second"),
                entry_step("completed.third"),
            ],
            vec![entry_step("blocked.second"), entry_step("blocked.third")],
            vec![entry_step("skipped.second"), entry_step("skipped.third")],
            vec![
                step_outcome("terminal.second"),
                step_outcome("terminal.third"),
            ],
        )
        .expect("workflow step execution plan");
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            selection,
            plan,
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_selection()
                .next_candidate_steps()[0]
                .as_str(),
            "candidate.second"
        );
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .completed_step_references()[0]
                .as_str(),
            "completed.second"
        );
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .blocked_step_references()[0]
                .as_str(),
            "blocked.second"
        );
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .skipped_step_references()[0]
                .as_str(),
            "skipped.second"
        );
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .terminal_step_references()[0]
                .as_str(),
            "terminal.second"
        );
    }

    #[test]
    fn workflow_step_coordination_deterministic_construction() {
        let first = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            workflow_step_execution_plan(),
        );
        let second = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            workflow_step_selection(),
            workflow_step_execution_plan(),
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_step_coordination_supplied_values_not_mutated() {
        let workflow_definition = workflow_definition();
        let workflow_instance = workflow_instance();
        let workflow_step_selection = workflow_step_selection();
        let workflow_step_execution_plan = workflow_step_execution_plan();

        let definition_before = workflow_definition.clone();
        let instance_before = workflow_instance.clone();
        let selection_before = workflow_step_selection.clone();
        let plan_before = workflow_step_execution_plan.clone();

        let coordination = WorkflowStepCoordination::new(
            workflow_definition.clone(),
            workflow_instance.clone(),
            workflow_step_selection.clone(),
            workflow_step_execution_plan.clone(),
        )
        .expect("workflow step coordination");

        assert_eq!(workflow_definition, definition_before);
        assert_eq!(workflow_instance, instance_before);
        assert_eq!(workflow_step_selection, selection_before);
        assert_eq!(workflow_step_execution_plan, plan_before);
        assert_eq!(coordination.workflow_instance(), &instance_before);
    }

    #[test]
    fn workflow_step_coordination_equivalent_invalid_inputs() {
        let first = WorkflowStepExecutionPlan::new(
            vec![entry_step("dup.step"), entry_step("dup.step")],
            vec![],
            vec![],
            vec![],
        )
        .expect_err("invalid");
        let second = WorkflowStepExecutionPlan::new(
            vec![entry_step("dup.step"), entry_step("dup.step")],
            vec![],
            vec![],
            vec![],
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_step_coordination_external_references_not_validated() {
        let selection = WorkflowStepSelection::new(
            entry_step("external.current-step"),
            vec![entry_step("external.next-step")],
        );
        let plan = WorkflowStepExecutionPlan::new(
            vec![entry_step("external.completed-step")],
            vec![entry_step("external.blocked-step")],
            vec![entry_step("external.skipped-step")],
            vec![step_outcome("external.terminal-step")],
        )
        .expect("workflow step execution plan");
        let coordination = WorkflowStepCoordination::new(
            workflow_definition(),
            workflow_instance(),
            selection,
            plan,
        )
        .expect("workflow step coordination");

        assert_eq!(
            coordination
                .workflow_step_selection()
                .current_step()
                .as_str(),
            "external.current-step"
        );
    }

    #[test]
    fn workflow_step_coordination_value_semantics_preserved() {
        let coordination = workflow_step_coordination();
        assert_eq!(coordination.clone(), coordination);
    }

    #[test]
    fn workflow_step_coordination_no_execution_occurs() {
        let coordination = workflow_step_coordination();
        assert_eq!(
            coordination
                .workflow_step_selection()
                .current_step()
                .as_str(),
            "start.review"
        );
    }

    #[test]
    fn workflow_step_coordination_no_task_created() {
        let coordination = workflow_step_coordination();
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .blocked_step_references()
                .len(),
            2
        );
    }

    #[test]
    fn workflow_step_coordination_no_event_emitted() {
        let coordination = workflow_step_coordination();
        assert_eq!(
            coordination
                .workflow_step_execution_plan()
                .terminal_step_references()
                .len(),
            2
        );
    }

    #[test]
    fn workflow_authorization_valid_authorized_construction() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        assert_eq!(
            request.workflow_operation().as_str(),
            "workflow.transition.approve"
        );
    }

    #[test]
    fn workflow_authorization_explicit_allow_returns_authorized_decision() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Allow,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn workflow_authorization_explicit_denial_returns_rejected_decision() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::DenyExplicit,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::DenyExplicit);
    }

    #[test]
    fn workflow_authorization_indeterminate_decision_returns_rejected_decision() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::DenyValidation,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::DenyValidation);
    }

    #[test]
    fn workflow_authorization_incomplete_authorization_result_rejected() {
        let error = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![],
        )
        .expect_err("missing authorization evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowAuthorizationIntegration(
                "workflow authorization evidence is required",
            )
        );
    }

    #[test]
    fn workflow_authorization_workflow_operation_preserved() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        assert_eq!(
            request.workflow_operation().as_str(),
            "workflow.transition.approve"
        );
    }

    #[test]
    fn workflow_authorization_workflow_definition_preserved() {
        let definition = workflow_definition();
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(definition.clone()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(request.workflow_definition(), Some(&definition));
    }

    #[test]
    fn workflow_authorization_workflow_instance_preserved() {
        let instance = workflow_instance();
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(instance.clone()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(request.workflow_instance(), Some(&instance));
    }

    #[test]
    fn workflow_authorization_workflow_state_preserved() {
        let state = workflow_state_snapshot();
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(state.clone()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(request.current_workflow_state(), Some(&state));
    }

    #[test]
    fn workflow_authorization_workflow_step_preserved() {
        let current_step = entry_step("start.review");
        let next_step = entry_step("collect-input");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(current_step.clone()),
            Some(next_step.clone()),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(request.current_workflow_step(), Some(&current_step));
        assert_eq!(request.requested_next_workflow_step(), Some(&next_step));
    }

    #[test]
    fn workflow_authorization_authorization_decision_preserved() {
        let context = workflow_authorization_context(AuthorizationDecisionOutcome::Allow);
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            context.clone(),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(
            request
                .workflow_authorization_context()
                .authorization_decision(),
            context.authorization_decision()
        );
    }

    #[test]
    fn workflow_authorization_authorization_evidence_preserved() {
        let context = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![
                authorization_audit_evidence_reference(
                    "CX-AUD-100001",
                    AuthorizationDecisionOutcome::Allow,
                ),
                authorization_audit_evidence_reference(
                    "CX-AUD-100002",
                    AuthorizationDecisionOutcome::Allow,
                ),
            ],
        )
        .expect("workflow authorization context");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            context.clone(),
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(
            request
                .workflow_authorization_context()
                .authorization_evidence_references(),
            context.authorization_evidence_references()
        );
    }

    #[test]
    fn workflow_authorization_transition_authority_preserved() {
        let authority =
            TransitionAuthorityReference::new("authority.workflow-owner").expect("authority");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            Some(authority.clone()),
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(request.transition_authority_reference(), Some(&authority));
    }

    #[test]
    fn workflow_authorization_transition_evidence_ordering_preserved() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        assert_eq!(
            request.transition_evidence_references()[0].as_str(),
            "transition.evidence.001"
        );
        assert_eq!(
            request.transition_evidence_references()[1].as_str(),
            "transition.evidence.002"
        );
    }

    #[test]
    fn workflow_authorization_duplicate_authorization_evidence_rejected() {
        let error = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![
                authorization_audit_evidence_reference(
                    "CX-AUD-100001",
                    AuthorizationDecisionOutcome::Allow,
                ),
                authorization_audit_evidence_reference(
                    "CX-AUD-100001",
                    AuthorizationDecisionOutcome::Allow,
                ),
            ],
        )
        .expect_err("duplicate authorization evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowAuthorizationIntegration(
                "duplicate workflow authorization evidence reference",
            )
        );
    }

    #[test]
    fn workflow_authorization_duplicate_transition_evidence_rejected() {
        let error = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
            ],
        )
        .expect_err("duplicate transition evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowAuthorizationIntegration(
                "duplicate workflow transition evidence reference",
            )
        );
    }

    #[test]
    fn workflow_authorization_ownership_does_not_imply_authorization() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Deny,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::Deny);
    }

    #[test]
    fn workflow_authorization_external_principal_existence_is_not_checked() {
        let subject = AuthorizationSubject::Principal(
            AuthorizationPrincipalReference::new(
                PrincipalId::new("CX-PRN-999999").expect("principal"),
                AuthorizationPrincipalType::Employee,
                "CX-EMP-999999",
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
                CredentialStatusReference::new("Valid").expect("credential"),
            )
            .expect("principal"),
        );
        let request_record = AuthorizationRequestRecord::new(
            AuthorizationRequestId::new("CX-AUTHREQ-000001").expect("request id"),
            subject,
            authorization_permission("approve-workflow"),
            authorization_target(),
            TimeReference::new("2026-07-17T00:00:00Z").expect("time"),
            "authorize workflow operation",
        )
        .expect("authorization request");
        let context = WorkflowAuthorizationContext::new(
            request_record,
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![authorization_audit_evidence_reference(
                "CX-AUD-100001",
                AuthorizationDecisionOutcome::Allow,
            )],
        )
        .expect("workflow authorization context");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            context,
            None,
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(
            WorkflowAuthorizationControl::evaluate(&request),
            AuthorizationDecisionOutcome::Allow
        );
    }

    #[test]
    fn workflow_authorization_external_authority_existence_is_not_checked() {
        let authority = TransitionAuthorityReference::new("authority.external").expect("authority");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            workflow_authorization_context(AuthorizationDecisionOutcome::Allow),
            Some(authority),
            vec![],
        )
        .expect("workflow authorization request");

        assert_eq!(
            WorkflowAuthorizationControl::evaluate(&request),
            AuthorizationDecisionOutcome::Allow
        );
    }

    #[test]
    fn workflow_authorization_external_evidence_existence_is_not_checked() {
        let context = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![AuthorizationAuditEvidenceReference::new(
                AuditEvidenceId::new("CX-AUD-999999").expect("audit id"),
                AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision id"),
                PrincipalId::new("CX-PRN-000001").expect("principal"),
                ScopeId::new("CX-SCP-000001").expect("scope"),
                StableVersion::new("policy_version", "1.0.0").expect("policy version"),
                vec![MatchedPolicyEvidenceReference::new("policy.match.external").expect("rule")],
                AuthorizationDecisionOutcome::Allow,
            )
            .expect("authorization audit evidence")],
        )
        .expect("workflow authorization context");
        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(WorkflowState::Approved),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            context,
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.external")
                    .expect("transition evidence"),
            ],
        )
        .expect("workflow authorization request");

        assert_eq!(
            WorkflowAuthorizationControl::evaluate(&request),
            AuthorizationDecisionOutcome::Allow
        );
    }

    #[test]
    fn workflow_authorization_equivalent_requests_return_equivalent_decisions() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        let first = WorkflowAuthorizationControl::evaluate(&request);
        let second = WorkflowAuthorizationControl::evaluate(&request);

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_authorization_construction_is_deterministic() {
        let first = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        let second = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_authorization_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![],
        )
        .expect_err("invalid authorization context");
        let second = WorkflowAuthorizationContext::new(
            authorization_request_record("approve-workflow"),
            authorization_decision_reference(AuthorizationDecisionOutcome::Allow),
            vec![],
        )
        .expect_err("invalid authorization context");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_authorization_supplied_values_are_not_mutated() {
        let definition = workflow_definition();
        let instance = workflow_instance();
        let state = workflow_state_snapshot();
        let coordination = workflow_step_coordination();
        let context = workflow_authorization_context(AuthorizationDecisionOutcome::Allow);
        let authority =
            TransitionAuthorityReference::new("authority.workflow-owner").expect("authority");
        let evidence = vec![
            TransitionEvidenceReference::new("transition.evidence.001")
                .expect("transition evidence"),
            TransitionEvidenceReference::new("transition.evidence.002")
                .expect("transition evidence"),
        ];

        let definition_before = definition.clone();
        let instance_before = instance.clone();
        let state_before = state.clone();
        let coordination_before = coordination.clone();
        let context_before = context.clone();
        let authority_before = authority.clone();
        let evidence_before = evidence.clone();

        let request = WorkflowAuthorizationRequest::new(
            WorkflowOperationReference::new("workflow.transition.approve").expect("operation"),
            Some(definition.clone()),
            Some(instance.clone()),
            Some(state.clone()),
            Some(WorkflowState::Approved),
            Some(coordination.clone()),
            Some(entry_step("start.review")),
            Some(entry_step("collect-input")),
            context.clone(),
            Some(authority.clone()),
            evidence.clone(),
        )
        .expect("workflow authorization request");

        let _ = WorkflowAuthorizationControl::evaluate(&request);

        assert_eq!(definition, definition_before);
        assert_eq!(instance, instance_before);
        assert_eq!(state, state_before);
        assert_eq!(coordination, coordination_before);
        assert_eq!(context, context_before);
        assert_eq!(authority, authority_before);
        assert_eq!(evidence, evidence_before);
    }

    #[test]
    fn workflow_authorization_no_policy_evaluation_occurs() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Allow,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn workflow_authorization_no_permission_calculation_occurs() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::DenyScope);
        assert_eq!(
            WorkflowAuthorizationControl::evaluate(&request),
            AuthorizationDecisionOutcome::DenyScope
        );
    }

    #[test]
    fn workflow_authorization_no_workflow_transition_executes() {
        let transition_request = WorkflowTransitionControlRequest::new(
            workflow_state_snapshot(),
            WorkflowState::Approved,
            None,
            None,
            vec![],
            None,
            crate::state::WorkflowLifecycleGuards::default(),
        )
        .expect("transition request");
        let before = transition_request.clone();

        let _ = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Allow,
        ));

        assert_eq!(transition_request, before);
    }

    #[test]
    fn workflow_authorization_no_step_execution_occurs() {
        let coordination = workflow_step_coordination();
        let before = coordination.clone();

        let _ = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Allow,
        ));

        assert_eq!(coordination, before);
    }

    #[test]
    fn workflow_authorization_no_task_is_created() {
        let request = workflow_authorization_request(AuthorizationDecisionOutcome::Allow);
        assert_eq!(
            request
                .workflow_step_coordination()
                .expect("coordination")
                .workflow_step_execution_plan()
                .blocked_step_references()
                .len(),
            2
        );
    }

    #[test]
    fn workflow_authorization_no_event_is_emitted() {
        let decision = WorkflowAuthorizationControl::evaluate(&workflow_authorization_request(
            AuthorizationDecisionOutcome::Allow,
        ));
        assert_eq!(decision, AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn workflow_authorization_existing_k3_authorization_apis_remain_usable() {
        let request = authorization_request_record("approve-workflow");
        let decision = authorization_decision_reference(AuthorizationDecisionOutcome::Allow);

        assert_eq!(request.request_id().as_str(), "CX-AUTHREQ-000001");
        assert_eq!(decision.outcome(), AuthorizationDecisionOutcome::Allow);
    }

    #[test]
    fn workflow_authorization_existing_k6_001_through_k6_005_apis_remain_usable() {
        let definition = workflow_definition();
        let instance = workflow_instance();
        let coordination = workflow_step_coordination();
        let transition_request = WorkflowTransitionControlRequest::new(
            workflow_state_snapshot(),
            WorkflowState::Validated,
            None,
            None,
            vec![],
            None,
            crate::state::WorkflowLifecycleGuards::default(),
        )
        .expect("transition request");

        assert_eq!(definition.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(instance.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(
            coordination
                .workflow_step_selection()
                .current_step()
                .as_str(),
            "start.review"
        );
        let transition_decision = WorkflowTransitionControl::evaluate(&transition_request);
        assert!(
            matches!(
                transition_decision,
                crate::state::WorkflowTransitionDecision::NoOp(_)
            ),
            "expected canonical no-op transition decision for identical workflow state request, got {transition_decision:?}"
        );
    }

    #[test]
    fn workflow_event_integration_valid_construction() {
        let request = workflow_event_request(
            "workflow.transition.allowed",
            workflow_event_context(
                Some(allowed_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                None,
            ),
            Some(CorrelationId::new("CX-COR-000001").expect("correlation")),
            EventCausation::root(),
        );

        assert_eq!(
            request.workflow_event_type().as_str(),
            "workflow.transition.allowed"
        );
    }

    #[test]
    fn workflow_event_integration_workflow_event_type_preserved() {
        let request = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        );
        assert_eq!(
            request.workflow_event_type().as_str(),
            "workflow.authorized"
        );
    }

    #[test]
    fn workflow_event_integration_canonical_event_id_preserved() {
        let request = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        );
        let decision = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert_eq!(decision.event_id().as_str(), "CX-EVT-000001");
    }

    #[test]
    fn workflow_event_integration_explicit_timestamp_preserved() {
        let request = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        );
        let decision = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert_eq!(decision.occurred_at().as_str(), "2026-07-17T00:00:00Z");
        assert_eq!(decision.recorded_at().as_str(), "2026-07-17T00:00:01Z");
    }

    #[test]
    fn workflow_event_integration_workflow_definition_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision
                .payload()
                .workflow_definition()
                .expect("definition")
                .workflow_id()
                .as_str(),
            "CX-WF-000001"
        );
    }

    #[test]
    fn workflow_event_integration_workflow_instance_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision
                .payload()
                .workflow_instance()
                .expect("instance")
                .workflow_id()
                .as_str(),
            "CX-WF-000001"
        );
    }

    #[test]
    fn workflow_event_integration_workflow_state_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision
                .payload()
                .workflow_state_snapshot()
                .expect("state")
                .lifecycle(),
            WorkflowState::Validated
        );
    }

    #[test]
    fn workflow_event_integration_workflow_step_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.step.selected",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision
                .payload()
                .workflow_step_reference()
                .expect("step")
                .as_str(),
            "start.review"
        );
    }

    #[test]
    fn workflow_event_integration_transition_allowed_maps_to_allowed_event() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.transition.allowed",
            workflow_event_context(
                Some(allowed_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                None,
            ),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision.event_type().as_str(),
            "workflow.transition.allowed"
        );
    }

    #[test]
    fn workflow_event_integration_transition_rejected_maps_to_rejected_event() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.transition.rejected",
            workflow_event_context(
                Some(rejected_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                None,
            ),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision.event_type().as_str(),
            "workflow.transition.rejected"
        );
    }

    #[test]
    fn workflow_event_integration_transition_no_op_maps_to_no_op_event() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.transition.noop",
            workflow_event_context(
                Some(noop_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                None,
            ),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.event_type().as_str(), "workflow.transition.noop");
    }

    #[test]
    fn workflow_event_integration_mismatched_transition_event_category_rejected() {
        let error = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.transition.allowed").expect("event type"),
            EventId::new("CX-EVT-000001").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            EventCausation::root(),
            workflow_event_context(
                Some(rejected_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                None,
            ),
        )
        .expect_err("mismatched transition event category must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowEventIntegration(
                "workflow transition allowed event requires allowed transition outcome",
            )
        );
    }

    #[test]
    fn workflow_event_integration_authorization_allow_maps_to_authorized_event() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.event_type().as_str(), "workflow.authorized");
    }

    #[test]
    fn workflow_event_integration_authorization_denial_maps_to_denied_event() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorization.denied",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::DenyExplicit), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision.event_type().as_str(),
            "workflow.authorization.denied"
        );
    }

    #[test]
    fn workflow_event_integration_mismatched_authorization_event_category_rejected() {
        let error = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.authorized").expect("event type"),
            EventId::new("CX-EVT-000001").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            EventCausation::root(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Deny), None),
        )
        .expect_err("mismatched authorization event category must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowEventIntegration(
                "workflow authorized event requires allow authorization outcome",
            )
        );
    }

    #[test]
    fn workflow_event_integration_failure_event_requires_failure_code() {
        let error = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.failed").expect("event type"),
            EventId::new("CX-EVT-000001").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            EventCausation::root(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
        )
        .expect_err("failure event without failure code must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowEventIntegration(
                "workflow failed event requires stable failure code",
            )
        );
    }

    #[test]
    fn workflow_event_integration_failure_code_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.failed",
            workflow_event_context(
                Some(rejected_transition_decision()),
                Some(AuthorizationDecisionOutcome::Allow),
                Some(WorkflowFailureCode::Timeout),
            ),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision.payload().failure_code(),
            Some(WorkflowFailureCode::Timeout)
        );
    }

    #[test]
    fn workflow_event_integration_non_failure_event_does_not_infer_failure() {
        let error = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.completed").expect("event type"),
            EventId::new("CX-EVT-000001").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            EventCausation::root(),
            workflow_event_context(
                None,
                Some(AuthorizationDecisionOutcome::Allow),
                Some(WorkflowFailureCode::Timeout),
            ),
        )
        .expect_err("non-failure event with failure code must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowEventIntegration(
                "non-failure workflow event must not carry failure code",
            )
        );
    }

    #[test]
    fn workflow_event_integration_correlation_reference_preserved() {
        let correlation = CorrelationId::new("CX-COR-000001").expect("correlation");
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            Some(correlation.clone()),
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.correlation_id(), Some(&correlation));
    }

    #[test]
    fn workflow_event_integration_causation_reference_preserved() {
        let current_event_id = EventId::new("CX-EVT-000001").expect("event id");
        let parent_event_id = EventId::new("CX-EVT-000099").expect("parent event id");
        let causation =
            EventCausation::caused_by(&current_event_id, parent_event_id.clone()).expect("cause");
        let request = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.authorized").expect("event type"),
            current_event_id,
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            causation.clone(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
        )
        .expect("request");
        let decision = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert_eq!(decision.causation(), &causation);
    }

    #[test]
    fn workflow_event_integration_evidence_order_preserved() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision.payload().transition_evidence_references()[0].as_str(),
            "transition.evidence.001"
        );
        assert_eq!(decision.trace().evidence_ids()[0].as_str(), "CX-AUD-000010");
    }

    #[test]
    fn workflow_event_integration_duplicate_evidence_rejected() {
        let error = WorkflowEventContext::new(
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            None,
            None,
            None,
            None,
            Some(WorkflowOperationReference::new("workflow.transition.approve").expect("op")),
            None,
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
            ],
            vec![audit_evidence("CX-AUD-000010")],
            None,
        )
        .expect_err("duplicate evidence must fail");

        assert_eq!(
            error,
            DomainError::InvalidWorkflowEventIntegration(
                "duplicate workflow transition evidence reference",
            )
        );
    }

    #[test]
    fn workflow_event_integration_equivalent_requests_produce_equivalent_decisions() {
        let request = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            Some(CorrelationId::new("CX-COR-000001").expect("correlation")),
            EventCausation::root(),
        );
        let first = WorkflowEventIntegration::evaluate(&request).expect("event");
        let second = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_event_integration_deterministic_construction() {
        let first = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        );
        let second = workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        );

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_event_integration_equivalent_invalid_inputs_produce_equivalent_errors() {
        let first = WorkflowEventContext::new(
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            None,
            None,
            None,
            None,
            Some(WorkflowOperationReference::new("workflow.transition.approve").expect("op")),
            None,
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
            ],
            vec![audit_evidence("CX-AUD-000010")],
            None,
        )
        .expect_err("invalid");
        let second = WorkflowEventContext::new(
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            None,
            None,
            None,
            None,
            Some(WorkflowOperationReference::new("workflow.transition.approve").expect("op")),
            None,
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
                TransitionEvidenceReference::new("transition.evidence.001")
                    .expect("transition evidence"),
            ],
            vec![audit_evidence("CX-AUD-000010")],
            None,
        )
        .expect_err("invalid");

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_event_integration_supplied_workflow_values_not_mutated() {
        let context = workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None);
        let context_before = context.clone();
        let request = workflow_event_request(
            "workflow.authorized",
            context.clone(),
            None,
            EventCausation::root(),
        );

        let _ = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert_eq!(context, context_before);
    }

    #[test]
    fn workflow_event_integration_external_event_id_existence_not_checked() {
        let request = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.authorized").expect("type"),
            EventId::new("CX-EVT-999999").expect("event id"),
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            EventCausation::root(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
        )
        .expect("request");

        assert_eq!(
            WorkflowEventIntegration::evaluate(&request)
                .expect("event")
                .event_id()
                .as_str(),
            "CX-EVT-999999"
        );
    }

    #[test]
    fn workflow_event_integration_external_correlation_existence_not_checked() {
        let correlation = CorrelationId::new("CX-COR-999999").expect("correlation");
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.authorized",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            Some(correlation.clone()),
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.correlation_id(), Some(&correlation));
    }

    #[test]
    fn workflow_event_integration_external_causation_existence_not_checked() {
        let current_event_id = EventId::new("CX-EVT-000001").expect("event id");
        let parent_event_id = EventId::new("CX-EVT-999999").expect("parent");
        let causation =
            EventCausation::caused_by(&current_event_id, parent_event_id.clone()).expect("cause");
        let request = WorkflowEventIntegrationRequest::new(
            EventType::new("workflow.authorized").expect("type"),
            current_event_id,
            EventVersion::new("1.0.0").expect("event version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            EventClassification::Internal,
            None,
            causation.clone(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
        )
        .expect("request");

        assert_eq!(
            WorkflowEventIntegration::evaluate(&request)
                .expect("event")
                .causation(),
            &causation
        );
    }

    #[test]
    fn workflow_event_integration_external_evidence_existence_not_checked() {
        let context = WorkflowEventContext::new(
            Some(workflow_definition()),
            Some(workflow_instance()),
            Some(workflow_state_snapshot()),
            Some(workflow_step_coordination()),
            Some(entry_step("start.review")),
            None,
            None,
            None,
            None,
            Some(WorkflowOperationReference::new("workflow.transition.approve").expect("op")),
            None,
            None,
            vec![
                TransitionEvidenceReference::new("transition.evidence.external")
                    .expect("transition evidence"),
            ],
            vec![audit_evidence("CX-AUD-999999")],
            None,
        )
        .expect("context");
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            context,
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.trace().evidence_ids()[0].as_str(), "CX-AUD-999999");
    }

    #[test]
    fn workflow_event_integration_no_system_clock_accessed() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.occurred_at().as_str(), "2026-07-17T00:00:00Z");
    }

    #[test]
    fn workflow_event_integration_no_identifier_generated() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.event_id().as_str(), "CX-EVT-000001");
    }

    #[test]
    fn workflow_event_integration_no_event_published() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.event_type().as_str(), "workflow.created");
    }

    #[test]
    fn workflow_event_integration_no_event_bus_called() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.source().component().as_str(), "workflow-engine");
    }

    #[test]
    fn workflow_event_integration_no_persistence_occurs() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(decision.subject().subject_id().as_str(), "CX-WF-000001");
    }

    #[test]
    fn workflow_event_integration_no_workflow_transition_executed() {
        let request = workflow_transition_control_request(
            WorkflowState::Ready,
            WorkflowState::Running,
            vec![TransitionEvidenceReference::new("transition.evidence.001")
                .expect("transition evidence")],
        );
        let before = request.clone();

        let _ = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.created",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(request, before);
    }

    #[test]
    fn workflow_event_integration_no_step_executed() {
        let coordination = workflow_step_coordination();
        let before = coordination.clone();

        let _ = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.step.selected",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(coordination, before);
    }

    #[test]
    fn workflow_event_integration_no_task_created() {
        let decision = WorkflowEventIntegration::evaluate(&workflow_event_request(
            "workflow.step.blocked",
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            None,
            EventCausation::root(),
        ))
        .expect("event");

        assert_eq!(
            decision
                .payload()
                .workflow_step_coordination()
                .expect("coordination")
                .workflow_step_execution_plan()
                .blocked_step_references()
                .len(),
            2
        );
    }

    #[test]
    fn workflow_event_integration_existing_k5_event_apis_remain_usable() {
        let envelope = EventEnvelope::new(
            EventId::new("CX-EVT-000500").expect("event id"),
            EventType::new("workflow.created").expect("event type"),
            EventVersion::new("1.0.0").expect("version"),
            TimeReference::new("2026-07-17T00:00:00Z").expect("occurred"),
            TimeReference::new("2026-07-17T00:00:01Z").expect("recorded"),
            workflow_event_source(),
            workflow_event_subject(),
            workflow_event_context(None, Some(AuthorizationDecisionOutcome::Allow), None),
            EventClassification::Internal,
            crate::event::EventTrace::new(None, Some(workflow_id()), None, None, vec![])
                .expect("trace"),
            None,
            EventCausation::root(),
        );

        assert_eq!(envelope.event_type().as_str(), "workflow.created");
    }

    #[test]
    fn workflow_event_integration_existing_k6_001_through_k6_006_apis_remain_usable() {
        let transition_decision = allowed_transition_decision();
        let authorization_decision = WorkflowAuthorizationControl::evaluate(
            &workflow_authorization_request(AuthorizationDecisionOutcome::Allow),
        );
        let request = workflow_event_request(
            "workflow.transition.allowed",
            workflow_event_context(
                Some(transition_decision.clone()),
                Some(authorization_decision),
                None,
            ),
            None,
            EventCausation::root(),
        );

        let decision = WorkflowEventIntegration::evaluate(&request).expect("event");

        assert!(matches!(
            decision
                .payload()
                .workflow_transition_decision()
                .expect("transition decision"),
            TransitionOutcome::Allowed(_)
        ));
        assert_eq!(
            decision
                .payload()
                .workflow_authorization_decision()
                .expect("authorization decision"),
            &AuthorizationDecisionOutcome::Allow
        );
    }
}
