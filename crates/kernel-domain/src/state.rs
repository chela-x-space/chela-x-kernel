use crate::identifier::{
    AgentId, DecisionAuthorityId, DecisionId, EnterpriseId, HumanId, NonEmptyText,
    OrganizationUnitId, OwnershipId, ProjectId, StableVersion, WorkspaceId,
};
use crate::lifecycle::{
    AgentLifecycle, DecisionRecordStatus, DelegationLifecycle, EnterpriseLifecycle, HumanLifecycle,
    OrganizationalUnitLifecycle, OwnershipLifecycle, ProjectLifecycle, WorkflowState,
    WorkspaceLifecycle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateSequence(u64);

impl StateSequence {
    pub fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionReasonReference(NonEmptyText);

impl TransitionReasonReference {
    pub fn new(value: impl Into<String>) -> crate::errors::DomainResult<Self> {
        Ok(Self(NonEmptyText::new("transition_reason", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionAuthorityReference(NonEmptyText);

impl TransitionAuthorityReference {
    pub fn new(value: impl Into<String>) -> crate::errors::DomainResult<Self> {
        Ok(Self(NonEmptyText::new("transition_authority", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionEvidenceReference(NonEmptyText);

impl TransitionEvidenceReference {
    pub fn new(value: impl Into<String>) -> crate::errors::DomainResult<Self> {
        Ok(Self(NonEmptyText::new("transition_evidence", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSnapshot<I, S> {
    subject_id: I,
    state: S,
    sequence: StateSequence,
}

impl<I, S> StateSnapshot<I, S> {
    pub fn new(subject_id: I, state: S, sequence: StateSequence) -> Self {
        Self {
            subject_id,
            state,
            sequence,
        }
    }

    pub fn subject_id(&self) -> &I {
        &self.subject_id
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentStateSnapshot {
    agent_id: AgentId,
    enterprise_id: EnterpriseId,
    namespace: crate::identifier::EnglishNamespace,
    identity_version: StableVersion,
    lifecycle: AgentLifecycle,
    sequence: StateSequence,
}

impl AgentStateSnapshot {
    pub fn new(
        agent_id: AgentId,
        enterprise_id: EnterpriseId,
        namespace: crate::identifier::EnglishNamespace,
        identity_version: StableVersion,
        lifecycle: AgentLifecycle,
        sequence: StateSequence,
    ) -> Self {
        Self {
            agent_id,
            enterprise_id,
            namespace,
            identity_version,
            lifecycle,
            sequence,
        }
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn namespace(&self) -> &crate::identifier::EnglishNamespace {
        &self.namespace
    }

    pub fn identity_version(&self) -> &StableVersion {
        &self.identity_version
    }

    pub fn lifecycle(&self) -> AgentLifecycle {
        self.lifecycle
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowStateSnapshot {
    workflow_id: crate::identifier::WorkflowId,
    ownership_path: crate::ownership::OwnershipPath,
    definition_version: StableVersion,
    lifecycle: WorkflowState,
    sequence: StateSequence,
}

impl WorkflowStateSnapshot {
    pub fn new(
        workflow_id: crate::identifier::WorkflowId,
        ownership_path: crate::ownership::OwnershipPath,
        definition_version: StableVersion,
        lifecycle: WorkflowState,
        sequence: StateSequence,
    ) -> Self {
        Self {
            workflow_id,
            ownership_path,
            definition_version,
            lifecycle,
            sequence,
        }
    }

    pub fn workflow_id(&self) -> &crate::identifier::WorkflowId {
        &self.workflow_id
    }

    pub fn ownership_path(&self) -> &crate::ownership::OwnershipPath {
        &self.ownership_path
    }

    pub fn definition_version(&self) -> &StableVersion {
        &self.definition_version
    }

    pub fn lifecycle(&self) -> WorkflowState {
        self.lifecycle
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTransitionControlRequest {
    current_workflow_state_snapshot: WorkflowStateSnapshot,
    requested_target_workflow_state: WorkflowState,
    transition_reason_reference: Option<TransitionReasonReference>,
    transition_authority_reference: Option<TransitionAuthorityReference>,
    transition_evidence_references: Vec<TransitionEvidenceReference>,
    failure_code: Option<WorkflowFailureCode>,
    workflow_lifecycle_guards: WorkflowLifecycleGuards,
}

impl WorkflowTransitionControlRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        current_workflow_state_snapshot: WorkflowStateSnapshot,
        requested_target_workflow_state: WorkflowState,
        transition_reason_reference: Option<TransitionReasonReference>,
        transition_authority_reference: Option<TransitionAuthorityReference>,
        transition_evidence_references: Vec<TransitionEvidenceReference>,
        failure_code: Option<WorkflowFailureCode>,
        workflow_lifecycle_guards: WorkflowLifecycleGuards,
    ) -> crate::errors::DomainResult<Self> {
        if requested_target_workflow_state == WorkflowState::Failed && failure_code.is_none() {
            return Err(
                crate::errors::DomainError::InvalidWorkflowTransitionControl(
                    "workflow failure target requires stable failure code",
                ),
            );
        }

        if requested_target_workflow_state != WorkflowState::Failed && failure_code.is_some() {
            return Err(
                crate::errors::DomainError::InvalidWorkflowTransitionControl(
                    "non-failure workflow transition must not carry failure code",
                ),
            );
        }

        for (index, evidence) in transition_evidence_references.iter().enumerate() {
            if transition_evidence_references[..index]
                .iter()
                .any(|prior| prior == evidence)
            {
                return Err(
                    crate::errors::DomainError::InvalidWorkflowTransitionControl(
                        "duplicate workflow transition evidence reference",
                    ),
                );
            }
        }

        Ok(Self {
            current_workflow_state_snapshot,
            requested_target_workflow_state,
            transition_reason_reference,
            transition_authority_reference,
            transition_evidence_references,
            failure_code,
            workflow_lifecycle_guards,
        })
    }

    pub fn current_workflow_state_snapshot(&self) -> &WorkflowStateSnapshot {
        &self.current_workflow_state_snapshot
    }

    pub fn requested_target_workflow_state(&self) -> WorkflowState {
        self.requested_target_workflow_state
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

    pub fn failure_code(&self) -> Option<WorkflowFailureCode> {
        self.failure_code
    }

    pub fn workflow_lifecycle_guards(&self) -> &WorkflowLifecycleGuards {
        &self.workflow_lifecycle_guards
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRequest<I, S> {
    subject_id: I,
    current_state: S,
    requested_state: S,
    reason: Option<TransitionReasonReference>,
    authority: Option<TransitionAuthorityReference>,
    evidence: Option<TransitionEvidenceReference>,
    sequence: StateSequence,
}

impl<I, S> TransitionRequest<I, S> {
    pub fn new(
        subject_id: I,
        current_state: S,
        requested_state: S,
        reason: Option<TransitionReasonReference>,
        authority: Option<TransitionAuthorityReference>,
        evidence: Option<TransitionEvidenceReference>,
        sequence: StateSequence,
    ) -> Self {
        Self {
            subject_id,
            current_state,
            requested_state,
            reason,
            authority,
            evidence,
            sequence,
        }
    }

    pub fn subject_id(&self) -> &I {
        &self.subject_id
    }

    pub fn current_state(&self) -> &S {
        &self.current_state
    }

    pub fn requested_state(&self) -> &S {
        &self.requested_state
    }

    pub fn reason(&self) -> Option<&TransitionReasonReference> {
        self.reason.as_ref()
    }

    pub fn authority(&self) -> Option<&TransitionAuthorityReference> {
        self.authority.as_ref()
    }

    pub fn evidence(&self) -> Option<&TransitionEvidenceReference> {
        self.evidence.as_ref()
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionRejectionReason {
    IllegalTransition,
    TerminalState,
    MissingAuthority,
    MissingEvidence,
    MissingReason,
    GuardFailed(&'static str),
    DeferredSemantics(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedTransition<I, S> {
    subject_id: I,
    from: S,
    to: S,
    sequence: StateSequence,
    reason: Option<TransitionReasonReference>,
    authority: Option<TransitionAuthorityReference>,
    evidence: Option<TransitionEvidenceReference>,
}

impl<I, S> AllowedTransition<I, S> {
    fn from_request(request: TransitionRequest<I, S>) -> Self {
        Self {
            subject_id: request.subject_id,
            from: request.current_state,
            to: request.requested_state,
            sequence: request.sequence,
            reason: request.reason,
            authority: request.authority,
            evidence: request.evidence,
        }
    }

    pub fn subject_id(&self) -> &I {
        &self.subject_id
    }

    pub fn from(&self) -> &S {
        &self.from
    }

    pub fn to(&self) -> &S {
        &self.to
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }

    pub fn reason(&self) -> Option<&TransitionReasonReference> {
        self.reason.as_ref()
    }

    pub fn authority(&self) -> Option<&TransitionAuthorityReference> {
        self.authority.as_ref()
    }

    pub fn evidence(&self) -> Option<&TransitionEvidenceReference> {
        self.evidence.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedTransition<I, S> {
    subject_id: I,
    from: S,
    to: S,
    sequence: StateSequence,
    reason: TransitionRejectionReason,
}

impl<I, S> RejectedTransition<I, S> {
    fn from_request(request: TransitionRequest<I, S>, reason: TransitionRejectionReason) -> Self {
        Self {
            subject_id: request.subject_id,
            from: request.current_state,
            to: request.requested_state,
            sequence: request.sequence,
            reason,
        }
    }

    pub fn subject_id(&self) -> &I {
        &self.subject_id
    }

    pub fn from(&self) -> &S {
        &self.from
    }

    pub fn to(&self) -> &S {
        &self.to
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }

    pub fn reason(&self) -> &TransitionRejectionReason {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoOpTransition<I, S> {
    subject_id: I,
    state: S,
    sequence: StateSequence,
}

impl<I, S> NoOpTransition<I, S> {
    fn from_request(request: TransitionRequest<I, S>) -> Self {
        Self {
            subject_id: request.subject_id,
            state: request.current_state,
            sequence: request.sequence,
        }
    }

    pub fn subject_id(&self) -> &I {
        &self.subject_id
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn sequence(&self) -> StateSequence {
        self.sequence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionOutcome<I, S> {
    Allowed(AllowedTransition<I, S>),
    Rejected(RejectedTransition<I, S>),
    NoOp(NoOpTransition<I, S>),
}

macro_rules! define_domain_aliases {
    ($snapshot:ident, $request:ident, $outcome:ident, $allowed:ident, $rejected:ident, $noop:ident, $id:ty, $state:ty) => {
        pub type $snapshot = StateSnapshot<$id, $state>;
        pub type $request = TransitionRequest<$id, $state>;
        pub type $outcome = TransitionOutcome<$id, $state>;
        pub type $allowed = AllowedTransition<$id, $state>;
        pub type $rejected = RejectedTransition<$id, $state>;
        pub type $noop = NoOpTransition<$id, $state>;
    };
}

define_domain_aliases!(
    EnterpriseStateSnapshot,
    EnterpriseTransitionRequest,
    EnterpriseTransitionOutcome,
    EnterpriseAllowedTransition,
    EnterpriseRejectedTransition,
    EnterpriseNoOpTransition,
    EnterpriseId,
    EnterpriseLifecycle
);
define_domain_aliases!(
    WorkspaceStateSnapshot,
    WorkspaceTransitionRequest,
    WorkspaceTransitionOutcome,
    WorkspaceAllowedTransition,
    WorkspaceRejectedTransition,
    WorkspaceNoOpTransition,
    WorkspaceId,
    WorkspaceLifecycle
);
define_domain_aliases!(
    ProjectStateSnapshot,
    ProjectTransitionRequest,
    ProjectTransitionOutcome,
    ProjectAllowedTransition,
    ProjectRejectedTransition,
    ProjectNoOpTransition,
    ProjectId,
    ProjectLifecycle
);
define_domain_aliases!(
    OrganizationalUnitStateSnapshot,
    OrganizationalUnitTransitionRequest,
    OrganizationalUnitTransitionOutcome,
    OrganizationalUnitAllowedTransition,
    OrganizationalUnitRejectedTransition,
    OrganizationalUnitNoOpTransition,
    OrganizationUnitId,
    OrganizationalUnitLifecycle
);
define_domain_aliases!(
    OwnershipStateSnapshot,
    OwnershipTransitionRequest,
    OwnershipTransitionOutcome,
    OwnershipAllowedTransition,
    OwnershipRejectedTransition,
    OwnershipNoOpTransition,
    OwnershipId,
    OwnershipLifecycle
);
define_domain_aliases!(
    HumanStateSnapshot,
    HumanTransitionRequest,
    HumanTransitionOutcome,
    HumanAllowedTransition,
    HumanRejectedTransition,
    HumanNoOpTransition,
    HumanId,
    HumanLifecycle
);
define_domain_aliases!(
    AgentTransitionStateSnapshot,
    AgentTransitionRequest,
    AgentTransitionOutcome,
    AgentAllowedTransition,
    AgentRejectedTransition,
    AgentNoOpTransition,
    AgentId,
    AgentLifecycle
);
define_domain_aliases!(
    DecisionStateSnapshot,
    DecisionTransitionRequest,
    DecisionTransitionOutcome,
    DecisionAllowedTransition,
    DecisionRejectedTransition,
    DecisionNoOpTransition,
    DecisionId,
    DecisionRecordStatus
);
define_domain_aliases!(
    DelegationStateSnapshot,
    DelegationTransitionRequest,
    DelegationTransitionOutcome,
    DelegationAllowedTransition,
    DelegationRejectedTransition,
    DelegationNoOpTransition,
    crate::identifier::DelegationId,
    DelegationLifecycle
);
define_domain_aliases!(
    WorkflowTransitionStateSnapshot,
    WorkflowTransitionRequest,
    WorkflowTransitionOutcome,
    WorkflowAllowedTransition,
    WorkflowRejectedTransition,
    WorkflowNoOpTransition,
    crate::identifier::WorkflowId,
    WorkflowState
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EnterpriseLifecycleGuards {
    pub has_owner: bool,
    pub has_name: bool,
    pub has_registry_metadata: bool,
    pub unresolved_critical_projects: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkspaceLifecycleGuards {
    pub has_owner: bool,
    pub has_access_boundary: bool,
    pub active_projects_attached: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProjectLifecycleGuards {
    pub has_owner: bool,
    pub has_objective: bool,
    pub has_workspace_link: bool,
    pub parent_workspace_active: bool,
    pub success_criteria_met: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OrganizationalUnitLifecycleGuards {
    pub has_owner: bool,
    pub has_capability_statement: bool,
    pub has_unit_type: bool,
    pub merge_target_same_enterprise: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OwnershipLifecycleGuards {
    pub subject_exists: bool,
    pub new_owner_is_distinct: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HumanLifecycleGuards {
    pub passed_evaluation: bool,
    pub completed_training: bool,
    pub certification_approved: bool,
    pub registration_completed: bool,
    pub assignment_ready: bool,
    pub promotion_approved: bool,
    pub transfer_approved: bool,
    pub maintenance_completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AgentLifecycleGuards {
    pub registration_complete: bool,
    pub verification_complete: bool,
    pub approval_complete: bool,
    pub valid_ownership: bool,
    pub valid_permissions: bool,
    pub current_supervision: bool,
    pub trust_valid: bool,
    pub lease_valid: bool,
    pub underlying_failure_resolved: bool,
    pub unresolved_obligations: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DecisionLifecycleGuards {
    pub validation_complete: bool,
    pub approval_complete: bool,
    pub successor_decision_id: Option<DecisionId>,
    pub retirement_authority_id: Option<DecisionAuthorityId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DelegationLifecycleGuards {
    pub policy_valid: bool,
    pub authorization_valid: bool,
    pub eligible_delegator: bool,
    pub eligible_delegate: bool,
    pub scope_compatible: bool,
    pub separation_of_duties_satisfied: bool,
    pub acceptance_required: bool,
    pub acceptance_recorded: bool,
    pub upstream_valid: bool,
    pub expiration_rule_present: bool,
    pub time_valid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkflowLifecycleGuards {
    pub policy_valid: bool,
    pub authorization_valid: bool,
    pub delegation_valid: bool,
    pub decision_valid: bool,
    pub scope_valid: bool,
    pub participants_valid: bool,
    pub audit_evidence: Option<TransitionEvidenceReference>,
    pub upstream_outcomes_allow: bool,
    pub retry_limit_respected: bool,
    pub recovery_revalidated: bool,
    pub failure_code: Option<WorkflowFailureCode>,
}

pub type WorkflowTransitionDecision = WorkflowTransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkflowTransitionControl;

impl WorkflowTransitionControl {
    pub fn evaluate(request: &WorkflowTransitionControlRequest) -> WorkflowTransitionDecision {
        let transition_evidence = request.transition_evidence_references.first().cloned();
        let mut workflow_lifecycle_guards = request.workflow_lifecycle_guards.clone();

        workflow_lifecycle_guards.failure_code = request.failure_code;

        let current_snapshot = request.current_workflow_state_snapshot();
        let provisional_request = WorkflowTransitionRequest::new(
            current_snapshot.workflow_id().clone(),
            current_snapshot.lifecycle(),
            request.requested_target_workflow_state(),
            request.transition_reason_reference().cloned(),
            request.transition_authority_reference().cloned(),
            transition_evidence.clone(),
            current_snapshot.sequence(),
        );

        match validate_workflow_transition(provisional_request, &workflow_lifecycle_guards) {
            TransitionOutcome::Allowed(_) => validate_workflow_transition(
                WorkflowTransitionRequest::new(
                    current_snapshot.workflow_id().clone(),
                    current_snapshot.lifecycle(),
                    request.requested_target_workflow_state(),
                    request.transition_reason_reference().cloned(),
                    request.transition_authority_reference().cloned(),
                    transition_evidence,
                    current_snapshot.sequence().next(),
                ),
                &workflow_lifecycle_guards,
            ),
            outcome => outcome,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowFailureCode {
    InvalidIdentifier,
    InvalidScope,
    TenantBoundaryViolation,
    SecurityClassificationViolation,
    PolicyDeny,
    AuthorizationDeny,
    DelegationInvalid,
    DecisionMissing,
    InvalidTransition,
    SodConflict,
    StageCycle,
    Timeout,
    AuditEvidenceMissing,
    Indeterminate,
}

impl WorkflowFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "WF_INVALID_IDENTIFIER",
            Self::InvalidScope => "WF_INVALID_SCOPE",
            Self::TenantBoundaryViolation => "WF_TENANT_BOUNDARY_VIOLATION",
            Self::SecurityClassificationViolation => "WF_SECURITY_CLASSIFICATION_VIOLATION",
            Self::PolicyDeny => "WF_POLICY_DENY",
            Self::AuthorizationDeny => "WF_AUTHORIZATION_DENY",
            Self::DelegationInvalid => "WF_DELEGATION_INVALID",
            Self::DecisionMissing => "WF_DECISION_MISSING",
            Self::InvalidTransition => "WF_INVALID_TRANSITION",
            Self::SodConflict => "WF_SOD_CONFLICT",
            Self::StageCycle => "WF_STAGE_CYCLE",
            Self::Timeout => "WF_TIMEOUT",
            Self::AuditEvidenceMissing => "WF_AUDIT_EVIDENCE_MISSING",
            Self::Indeterminate => "WF_INDETERMINATE",
        }
    }
}

fn allow<I, S>(request: TransitionRequest<I, S>) -> TransitionOutcome<I, S> {
    TransitionOutcome::Allowed(AllowedTransition::from_request(request))
}

fn reject<I, S>(
    request: TransitionRequest<I, S>,
    reason: TransitionRejectionReason,
) -> TransitionOutcome<I, S> {
    TransitionOutcome::Rejected(RejectedTransition::from_request(request, reason))
}

fn noop<I, S>(request: TransitionRequest<I, S>) -> TransitionOutcome<I, S> {
    TransitionOutcome::NoOp(NoOpTransition::from_request(request))
}

fn terminal_transition<S: Copy + Eq>(
    current: S,
    requested: S,
    is_terminal: bool,
) -> Option<TransitionRejectionReason> {
    if current == requested {
        None
    } else if is_terminal {
        Some(TransitionRejectionReason::TerminalState)
    } else {
        None
    }
}

pub fn validate_enterprise_transition(
    request: EnterpriseTransitionRequest,
    guards: EnterpriseLifecycleGuards,
) -> EnterpriseTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if let Some(reason) = terminal_transition(
        request.current_state,
        request.requested_state,
        request.current_state == EnterpriseLifecycle::Dissolved,
    ) {
        return reject(request, reason);
    }
    match (request.current_state, request.requested_state) {
        (EnterpriseLifecycle::Proposed, EnterpriseLifecycle::Active) => {
            if !guards.has_owner || !guards.has_name || !guards.has_registry_metadata {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "missing enterprise activation prerequisites",
                    ),
                );
            }
            allow(request)
        }
        (EnterpriseLifecycle::Active, EnterpriseLifecycle::Suspended) => {
            if guards.unresolved_critical_projects {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "enterprise suspension blocked by unresolved critical projects",
                    ),
                );
            }
            allow(request)
        }
        (EnterpriseLifecycle::Active, EnterpriseLifecycle::Dissolved)
        | (EnterpriseLifecycle::Suspended, EnterpriseLifecycle::Dissolved) => allow(request),
        (EnterpriseLifecycle::Suspended, EnterpriseLifecycle::Active) => reject(
            request,
            TransitionRejectionReason::DeferredSemantics(
                "enterprise reactivation is not defined in CES-B0-025.1",
            ),
        ),
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_workspace_transition(
    request: WorkspaceTransitionRequest,
    guards: WorkspaceLifecycleGuards,
) -> WorkspaceTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if request.current_state == WorkspaceLifecycle::Retired {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (WorkspaceLifecycle::Planned, WorkspaceLifecycle::Provisioning) => allow(request),
        (WorkspaceLifecycle::Provisioning, WorkspaceLifecycle::Active) => {
            if !guards.has_owner || !guards.has_access_boundary {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "workspace activation requires owner and access boundary",
                    ),
                );
            }
            allow(request)
        }
        (WorkspaceLifecycle::Active, WorkspaceLifecycle::Archived) => allow(request),
        (WorkspaceLifecycle::Archived, WorkspaceLifecycle::Retired) => {
            if guards.active_projects_attached {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "workspace retirement blocked by active attached projects",
                    ),
                );
            }
            allow(request)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_project_transition(
    request: ProjectTransitionRequest,
    guards: ProjectLifecycleGuards,
) -> ProjectTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if matches!(
        request.current_state,
        ProjectLifecycle::Completed | ProjectLifecycle::Cancelled
    ) {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (ProjectLifecycle::Draft, ProjectLifecycle::Approved) => {
            if !guards.has_objective || !guards.has_owner || !guards.has_workspace_link {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "project approval requires objective, owner, and workspace link",
                    ),
                );
            }
            allow(request)
        }
        (ProjectLifecycle::Approved, ProjectLifecycle::Active) => {
            if !guards.parent_workspace_active {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "project start requires an active parent workspace",
                    ),
                );
            }
            allow(request)
        }
        (ProjectLifecycle::Active, ProjectLifecycle::Paused) => allow(request),
        (ProjectLifecycle::Paused, ProjectLifecycle::Active) => {
            if !guards.parent_workspace_active {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "project resume requires an active parent workspace",
                    ),
                );
            }
            allow(request)
        }
        (ProjectLifecycle::Active, ProjectLifecycle::Completed) => {
            if !guards.success_criteria_met {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "project completion requires satisfied success criteria",
                    ),
                );
            }
            allow(request)
        }
        (ProjectLifecycle::Active, ProjectLifecycle::Cancelled)
        | (ProjectLifecycle::Paused, ProjectLifecycle::Cancelled) => allow(request),
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_organizational_unit_transition(
    request: OrganizationalUnitTransitionRequest,
    guards: OrganizationalUnitLifecycleGuards,
) -> OrganizationalUnitTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if matches!(
        request.current_state,
        OrganizationalUnitLifecycle::Merged | OrganizationalUnitLifecycle::Closed
    ) {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (OrganizationalUnitLifecycle::Proposed, OrganizationalUnitLifecycle::Established) => {
            allow(request)
        }
        (OrganizationalUnitLifecycle::Established, OrganizationalUnitLifecycle::Operating) => {
            if !guards.has_owner || !guards.has_capability_statement || !guards.has_unit_type {
                return reject(request, TransitionRejectionReason::GuardFailed("organizational unit operation requires owner, capability statement, and valid unit type"));
            }
            allow(request)
        }
        (OrganizationalUnitLifecycle::Established, OrganizationalUnitLifecycle::Closed)
        | (OrganizationalUnitLifecycle::Operating, OrganizationalUnitLifecycle::Closed) => {
            allow(request)
        }
        (OrganizationalUnitLifecycle::Operating, OrganizationalUnitLifecycle::Merged) => {
            if !guards.merge_target_same_enterprise {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "organizational unit merge requires same-enterprise target",
                    ),
                );
            }
            allow(request)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_ownership_transition(
    request: OwnershipTransitionRequest,
    guards: OwnershipLifecycleGuards,
) -> OwnershipTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if matches!(
        request.current_state,
        OwnershipLifecycle::Transferred | OwnershipLifecycle::Revoked | OwnershipLifecycle::Expired
    ) {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (OwnershipLifecycle::Draft, OwnershipLifecycle::Active) => {
            if !guards.subject_exists {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "ownership activation requires an existing governed subject",
                    ),
                );
            }
            allow(request)
        }
        (OwnershipLifecycle::Active, OwnershipLifecycle::Transferred) => {
            if !guards.new_owner_is_distinct {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "ownership transfer requires a distinct new owner",
                    ),
                );
            }
            allow(request)
        }
        (OwnershipLifecycle::Active, OwnershipLifecycle::Revoked)
        | (OwnershipLifecycle::Active, OwnershipLifecycle::Expired) => allow(request),
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_human_transition(
    request: HumanTransitionRequest,
    guards: HumanLifecycleGuards,
) -> HumanTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if request.current_state == HumanLifecycle::Archive {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (HumanLifecycle::Candidate, HumanLifecycle::Evaluation) => allow(request),
        (HumanLifecycle::Evaluation, HumanLifecycle::Training) => {
            if !guards.passed_evaluation {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "training requires successful evaluation",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Training, HumanLifecycle::Certification) => {
            if !guards.completed_training {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "certification requires completed training",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Certification, HumanLifecycle::Registration) => {
            if !guards.certification_approved {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "registration requires approved certification",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Registration, HumanLifecycle::Active) => {
            if !guards.registration_completed || !guards.assignment_ready {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "active status requires registration records and assignment readiness",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Active, HumanLifecycle::Promotion) => {
            if !guards.promotion_approved {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "promotion requires approved advancement evidence",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Promotion, HumanLifecycle::Transfer) => {
            if !guards.transfer_approved {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "transfer requires approved transfer evidence",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Transfer, HumanLifecycle::Maintenance) => allow(request),
        (HumanLifecycle::Maintenance, HumanLifecycle::Retirement) => {
            if !guards.maintenance_completed {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "retirement requires completed maintenance obligations",
                    ),
                );
            }
            allow(request)
        }
        (HumanLifecycle::Retirement, HumanLifecycle::Archive) => allow(request),
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_agent_transition(
    request: AgentTransitionRequest,
    guards: AgentLifecycleGuards,
) -> AgentTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if request.current_state == AgentLifecycle::Deleted {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (AgentLifecycle::Created, AgentLifecycle::Registered) => {
            if !guards.registration_complete {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "agent registration requires complete registration evidence",
                    ),
                );
            }
            allow(request)
        }
        (AgentLifecycle::Registered, AgentLifecycle::Verified) => {
            if !guards.verification_complete {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "agent verification requires verification evidence",
                    ),
                );
            }
            allow(request)
        }
        (AgentLifecycle::Verified, AgentLifecycle::Approved) => {
            if !guards.approval_complete {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "agent approval requires approval evidence",
                    ),
                );
            }
            allow(request)
        }
        (AgentLifecycle::Approved, AgentLifecycle::Active)
        | (AgentLifecycle::Paused, AgentLifecycle::Active)
        | (AgentLifecycle::Recovering, AgentLifecycle::Active) => {
            if !guards.valid_ownership
                || !guards.valid_permissions
                || !guards.current_supervision
                || !guards.trust_valid
                || !guards.lease_valid
            {
                return reject(request, TransitionRejectionReason::GuardFailed("agent activation requires ownership, permissions, supervision, trust, and lease validity"));
            }
            if request.current_state == AgentLifecycle::Recovering
                && !guards.underlying_failure_resolved
            {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "agent recovery activation requires resolved underlying failure",
                    ),
                );
            }
            allow(request)
        }
        (AgentLifecycle::Active, AgentLifecycle::Paused)
        | (AgentLifecycle::Active, AgentLifecycle::Suspended)
        | (AgentLifecycle::Paused, AgentLifecycle::Suspended) => allow(request),
        (AgentLifecycle::Suspended, AgentLifecycle::Recovering) => {
            if !guards.underlying_failure_resolved {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "agent recovery requires resolved underlying failure",
                    ),
                );
            }
            allow(request)
        }
        (AgentLifecycle::Recovering, AgentLifecycle::Registered) => {
            if !guards.underlying_failure_resolved
                || !guards.current_supervision
                || !guards.trust_valid
            {
                return reject(request, TransitionRejectionReason::GuardFailed("agent recovery to registered requires resolved failure, supervision, and trust validity"));
            }
            allow(request)
        }
        (AgentLifecycle::Active, AgentLifecycle::Retired)
        | (AgentLifecycle::Paused, AgentLifecycle::Retired)
        | (AgentLifecycle::Suspended, AgentLifecycle::Retired) => allow(request),
        (AgentLifecycle::Retired, AgentLifecycle::Deleted) => {
            if guards.unresolved_obligations {
                return reject(request, TransitionRejectionReason::GuardFailed("agent deletion requires resolved audit, recovery, and ownership obligations"));
            }
            allow(request)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_decision_transition(
    request: DecisionTransitionRequest,
    guards: &DecisionLifecycleGuards,
) -> DecisionTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if request.current_state == DecisionRecordStatus::Archived {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (DecisionRecordStatus::Draft, DecisionRecordStatus::PendingReview)
        | (DecisionRecordStatus::Draft, DecisionRecordStatus::PendingApproval) => allow(request),
        (DecisionRecordStatus::PendingReview, DecisionRecordStatus::PendingApproval) => {
            if !guards.validation_complete {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "decision approval review requires validation completion",
                    ),
                );
            }
            allow(request)
        }
        (DecisionRecordStatus::PendingReview, DecisionRecordStatus::Rejected)
        | (DecisionRecordStatus::PendingApproval, DecisionRecordStatus::Rejected) => {
            if request.reason.is_none() {
                return reject(request, TransitionRejectionReason::MissingReason);
            }
            if request.authority.is_none() {
                return reject(request, TransitionRejectionReason::MissingAuthority);
            }
            allow(request)
        }
        (DecisionRecordStatus::PendingApproval, DecisionRecordStatus::Approved) => {
            if !guards.validation_complete || !guards.approval_complete {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "decision approval requires validation and approval completion",
                    ),
                );
            }
            if request.authority.is_none() {
                return reject(request, TransitionRejectionReason::MissingAuthority);
            }
            if request.evidence.is_none() {
                return reject(request, TransitionRejectionReason::MissingEvidence);
            }
            allow(request)
        }
        (DecisionRecordStatus::Approved, DecisionRecordStatus::Executed) => {
            if request.evidence.is_none() {
                return reject(request, TransitionRejectionReason::MissingEvidence);
            }
            allow(request)
        }
        (DecisionRecordStatus::Approved, DecisionRecordStatus::Superseded)
        | (DecisionRecordStatus::Rejected, DecisionRecordStatus::Superseded)
        | (DecisionRecordStatus::Executed, DecisionRecordStatus::Superseded) => {
            if guards.successor_decision_id.is_none() && guards.retirement_authority_id.is_none() {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "decision supersession requires successor or retirement authority",
                    ),
                );
            }
            allow(request)
        }
        (DecisionRecordStatus::Rejected, DecisionRecordStatus::Archived)
        | (DecisionRecordStatus::Executed, DecisionRecordStatus::Archived)
        | (DecisionRecordStatus::Superseded, DecisionRecordStatus::Archived)
        | (DecisionRecordStatus::Approved, DecisionRecordStatus::Archived) => allow(request),
        (DecisionRecordStatus::Draft, DecisionRecordStatus::Executed) => {
            reject(request, TransitionRejectionReason::IllegalTransition)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_delegation_transition(
    request: DelegationTransitionRequest,
    guards: DelegationLifecycleGuards,
) -> DelegationTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    match (request.current_state, request.requested_state) {
        (DelegationLifecycle::Draft, DelegationLifecycle::Requested)
        | (DelegationLifecycle::Requested, DelegationLifecycle::PolicyValidated)
        | (DelegationLifecycle::Requested, DelegationLifecycle::Rejected)
        | (DelegationLifecycle::PolicyValidated, DelegationLifecycle::AuthorizationValidated)
        | (DelegationLifecycle::PolicyValidated, DelegationLifecycle::Rejected)
        | (DelegationLifecycle::PendingAcceptance, DelegationLifecycle::Accepted)
        | (DelegationLifecycle::PendingAcceptance, DelegationLifecycle::Rejected)
        | (DelegationLifecycle::Accepted, DelegationLifecycle::Rejected)
        | (DelegationLifecycle::Active, DelegationLifecycle::Suspended)
        | (DelegationLifecycle::Active, DelegationLifecycle::Revoked)
        | (DelegationLifecycle::Active, DelegationLifecycle::Expired)
        | (DelegationLifecycle::Active, DelegationLifecycle::Completed)
        | (DelegationLifecycle::Suspended, DelegationLifecycle::Revoked)
        | (DelegationLifecycle::Suspended, DelegationLifecycle::Expired)
        | (DelegationLifecycle::Completed, DelegationLifecycle::Archived)
        | (DelegationLifecycle::Rejected, DelegationLifecycle::Archived)
        | (DelegationLifecycle::Revoked, DelegationLifecycle::Archived)
        | (DelegationLifecycle::Expired, DelegationLifecycle::Archived) => allow(request),
        (DelegationLifecycle::AuthorizationValidated, DelegationLifecycle::PendingAcceptance)
        | (DelegationLifecycle::AuthorizationValidated, DelegationLifecycle::Active)
        | (DelegationLifecycle::Accepted, DelegationLifecycle::Active)
        | (DelegationLifecycle::Suspended, DelegationLifecycle::Active) => {
            if !guards.policy_valid
                || !guards.authorization_valid
                || !guards.eligible_delegator
                || !guards.eligible_delegate
                || !guards.scope_compatible
                || !guards.separation_of_duties_satisfied
                || !guards.upstream_valid
                || !guards.time_valid
                || !guards.expiration_rule_present
            {
                return reject(request, TransitionRejectionReason::GuardFailed("delegation activation or resume requires valid policy, authorization, eligibility, scope, SoD, upstream state, time bounds, and expiration rule"));
            }
            if guards.acceptance_required && !guards.acceptance_recorded {
                return reject(request, TransitionRejectionReason::GuardFailed("delegation activation requires recorded acceptance when acceptance is mandatory"));
            }
            allow(request)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

pub fn validate_workflow_transition(
    request: WorkflowTransitionRequest,
    guards: &WorkflowLifecycleGuards,
) -> WorkflowTransitionOutcome {
    if request.current_state == request.requested_state {
        return noop(request);
    }
    if request.current_state == WorkflowState::Archived {
        return reject(request, TransitionRejectionReason::TerminalState);
    }
    match (request.current_state, request.requested_state) {
        (WorkflowState::Draft, WorkflowState::Defined)
        | (WorkflowState::Defined, WorkflowState::Validated)
        | (WorkflowState::Defined, WorkflowState::Archived)
        | (WorkflowState::Validated, WorkflowState::Defined)
        | (WorkflowState::Validated, WorkflowState::Archived)
        | (WorkflowState::Approved, WorkflowState::Archived)
        | (WorkflowState::Ready, WorkflowState::Cancelled)
        | (WorkflowState::Ready, WorkflowState::Archived)
        | (WorkflowState::Running, WorkflowState::Paused)
        | (WorkflowState::Running, WorkflowState::Waiting)
        | (WorkflowState::Running, WorkflowState::Completed)
        | (WorkflowState::Running, WorkflowState::Cancelled)
        | (WorkflowState::Paused, WorkflowState::Cancelled)
        | (WorkflowState::Paused, WorkflowState::Archived)
        | (WorkflowState::Waiting, WorkflowState::Cancelled)
        | (WorkflowState::Waiting, WorkflowState::Archived)
        | (WorkflowState::Completed, WorkflowState::Archived)
        | (WorkflowState::Cancelled, WorkflowState::Archived) => allow(request),
        (WorkflowState::Validated, WorkflowState::Approved) => {
            if !guards.decision_valid {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "workflow approval requires valid upstream decision",
                    ),
                );
            }
            allow(request)
        }
        (WorkflowState::Approved, WorkflowState::Ready)
        | (WorkflowState::Ready, WorkflowState::Running)
        | (WorkflowState::Paused, WorkflowState::Running)
        | (WorkflowState::Waiting, WorkflowState::Running)
        | (WorkflowState::Failed, WorkflowState::Ready) => {
            if !guards.policy_valid
                || !guards.authorization_valid
                || !guards.delegation_valid
                || !guards.decision_valid
                || !guards.scope_valid
                || !guards.participants_valid
                || !guards.upstream_outcomes_allow
            {
                return reject(request, TransitionRejectionReason::GuardFailed("workflow start or resume requires valid policy, authorization, delegation, decision, scope, participants, and allowing upstream outcomes"));
            }
            if guards.audit_evidence.is_none() {
                return reject(request, TransitionRejectionReason::MissingEvidence);
            }
            if request.current_state == WorkflowState::Failed
                && (!guards.retry_limit_respected || !guards.recovery_revalidated)
            {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "workflow recovery to ready requires bounded retry and fresh revalidation",
                    ),
                );
            }
            allow(request)
        }
        (WorkflowState::Running, WorkflowState::Failed)
        | (WorkflowState::Waiting, WorkflowState::Failed) => {
            if guards.failure_code.is_none() {
                return reject(
                    request,
                    TransitionRejectionReason::GuardFailed(
                        "workflow failure transition requires a stable failure code",
                    ),
                );
            }
            allow(request)
        }
        _ => reject(request, TransitionRejectionReason::IllegalTransition),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        validate_agent_transition, validate_decision_transition, validate_delegation_transition,
        validate_enterprise_transition, validate_human_transition, validate_ownership_transition,
        validate_project_transition, validate_workflow_transition, validate_workspace_transition,
        AgentLifecycleGuards, DecisionLifecycleGuards, DelegationLifecycleGuards,
        EnterpriseLifecycleGuards, HumanLifecycleGuards, OwnershipLifecycleGuards,
        ProjectLifecycleGuards, StateSequence, TransitionAuthorityReference,
        TransitionEvidenceReference, TransitionOutcome, TransitionReasonReference,
        TransitionRejectionReason, WorkflowFailureCode, WorkflowLifecycleGuards,
        WorkflowStateSnapshot, WorkflowTransitionControl, WorkflowTransitionControlRequest,
        WorkspaceLifecycleGuards,
    };
    use crate::identifier::{
        AgentId, DecisionAuthorityId, DecisionId, DelegationId, EnterpriseId, HumanId, OwnershipId,
        ProjectId, StableVersion, WorkflowId, WorkspaceId,
    };
    use crate::lifecycle::{
        AgentLifecycle, DecisionRecordStatus, DelegationLifecycle, EnterpriseLifecycle,
        HumanLifecycle, OwnershipLifecycle, ProjectLifecycle, WorkflowState, WorkspaceLifecycle,
    };
    use crate::ownership::OwnershipPath;
    use crate::workflow::{
        WorkflowAuditEvidenceReference, WorkflowDefinition, WorkflowInstance,
        WorkflowLifecycleMapReference, WorkflowRecoveryReference, WorkflowRetryLimit,
        WorkflowRetryPolicyReference, WorkflowStepReference, WorkflowTerminalOutcomeReference,
    };

    fn sequence() -> StateSequence {
        StateSequence::new(1).expect("sequence")
    }

    fn workflow_transition_ownership() -> OwnershipPath {
        OwnershipPath::new(
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new("CX-PROJ-000001").expect("project")),
            None,
        )
        .expect("ownership path")
    }

    fn workflow_transition_snapshot(state: WorkflowState) -> WorkflowStateSnapshot {
        WorkflowStateSnapshot::new(
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            workflow_transition_ownership(),
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            state,
            sequence(),
        )
    }

    fn workflow_transition_guards() -> WorkflowLifecycleGuards {
        WorkflowLifecycleGuards {
            policy_valid: true,
            authorization_valid: true,
            delegation_valid: true,
            decision_valid: true,
            scope_valid: true,
            participants_valid: true,
            audit_evidence: Some(
                TransitionEvidenceReference::new("AUD-001").expect("audit evidence"),
            ),
            upstream_outcomes_allow: true,
            retry_limit_respected: true,
            recovery_revalidated: true,
            failure_code: None,
        }
    }

    fn workflow_transition_request(
        current_state: WorkflowState,
        requested_target_workflow_state: WorkflowState,
    ) -> WorkflowTransitionControlRequest {
        WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(current_state),
            requested_target_workflow_state,
            Some(TransitionReasonReference::new("operator request").expect("reason")),
            Some(TransitionAuthorityReference::new("authority.ref").expect("authority")),
            vec![TransitionEvidenceReference::new("AUD-001").expect("evidence")],
            None,
            workflow_transition_guards(),
        )
        .expect("transition request")
    }

    fn workflow_transition_request_without_guard_audit_evidence(
        current_state: WorkflowState,
        requested_target_workflow_state: WorkflowState,
    ) -> WorkflowTransitionControlRequest {
        WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(current_state),
            requested_target_workflow_state,
            Some(TransitionReasonReference::new("operator request").expect("reason")),
            Some(TransitionAuthorityReference::new("authority.ref").expect("authority")),
            vec![TransitionEvidenceReference::new("AUD-001").expect("evidence")],
            None,
            WorkflowLifecycleGuards {
                audit_evidence: None,
                ..workflow_transition_guards()
            },
        )
        .expect("transition request")
    }

    fn workflow_retry_limit() -> WorkflowRetryLimit {
        WorkflowRetryLimit::new(2).expect("retry limit")
    }

    fn workflow_retry_policy() -> WorkflowRetryPolicyReference {
        WorkflowRetryPolicyReference::new(
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            workflow_retry_limit(),
        )
    }

    fn workflow_recovery_reference() -> WorkflowRecoveryReference {
        WorkflowRecoveryReference::new("retry/manual-review", true).expect("recovery")
    }

    fn workflow_audit_evidence(id: &str) -> WorkflowAuditEvidenceReference {
        WorkflowAuditEvidenceReference::new(
            crate::identifier::AuditEvidenceId::new(id).expect("audit evidence id"),
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            vec![crate::identifier::PolicyId::new("CX-POL-000001").expect("policy")],
            vec![
                crate::identifier::AuthorizationDecisionId::new("CX-AUTHDEC-000001")
                    .expect("authorization decision"),
            ],
            vec![crate::identifier::DelegationId::new("CX-DEL-000001").expect("delegation")],
            vec![DecisionId::new("CX-DEC-000001").expect("decision")],
        )
        .expect("audit evidence")
    }

    fn workflow_definition() -> WorkflowDefinition {
        WorkflowDefinition::new(
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            crate::identifier::EnglishNamespace::new("workflow_namespace", "ops.approval-flow")
                .expect("namespace"),
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            workflow_transition_ownership(),
            WorkflowLifecycleMapReference::new("workflow.lifecycle.v1").expect("lifecycle map"),
            vec![WorkflowStepReference::new("start.review").expect("entry step")],
            vec![WorkflowTerminalOutcomeReference::new("completed").expect("terminal outcome")],
            vec![crate::identifier::PolicyId::new("CX-POL-000001").expect("policy")],
            Some(workflow_retry_policy()),
            Some(workflow_retry_limit()),
            Some(workflow_recovery_reference()),
            vec![workflow_audit_evidence("CX-AUD-000001")],
        )
        .expect("workflow definition")
    }

    fn workflow_instance(state: WorkflowState) -> WorkflowInstance {
        WorkflowInstance::new(
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            workflow_definition(),
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            workflow_transition_ownership(),
            workflow_transition_snapshot(state),
            workflow_audit_evidence("CX-AUD-000002"),
            Some(workflow_retry_policy()),
            Some(workflow_retry_limit()),
            Some(workflow_recovery_reference()),
            vec![workflow_audit_evidence("CX-AUD-000003")],
        )
        .expect("workflow instance")
    }

    #[test]
    fn enterprise_allows_activation_with_required_prerequisites_ces_b0_025_1() {
        let outcome = validate_enterprise_transition(
            super::EnterpriseTransitionRequest::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                EnterpriseLifecycle::Proposed,
                EnterpriseLifecycle::Active,
                None,
                None,
                None,
                sequence(),
            ),
            EnterpriseLifecycleGuards {
                has_owner: true,
                has_name: true,
                has_registry_metadata: true,
                unresolved_critical_projects: false,
            },
        );
        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn workspace_rejects_retirement_with_active_projects_ces_b0_025_2() {
        let outcome = validate_workspace_transition(
            super::WorkspaceTransitionRequest::new(
                WorkspaceId::new("CX-WS-000001").expect("workspace"),
                WorkspaceLifecycle::Archived,
                WorkspaceLifecycle::Retired,
                None,
                None,
                None,
                sequence(),
            ),
            WorkspaceLifecycleGuards {
                has_owner: true,
                has_access_boundary: true,
                active_projects_attached: true,
            },
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("workspace retirement blocked by active attached projects")
        ));
    }

    #[test]
    fn project_rejects_noop_transition_ces_b0_025_3() {
        let outcome = validate_project_transition(
            super::ProjectTransitionRequest::new(
                ProjectId::new("CX-PROJ-000001").expect("project"),
                ProjectLifecycle::Approved,
                ProjectLifecycle::Approved,
                None,
                None,
                None,
                sequence(),
            ),
            ProjectLifecycleGuards::default(),
        );
        assert!(matches!(outcome, TransitionOutcome::NoOp(_)));
    }

    #[test]
    fn ownership_rejects_terminal_reactivation_ces_b0_025_5() {
        let outcome = validate_ownership_transition(
            super::OwnershipTransitionRequest::new(
                OwnershipId::new("CX-OWN-000001").expect("ownership"),
                OwnershipLifecycle::Revoked,
                OwnershipLifecycle::Active,
                None,
                None,
                None,
                sequence(),
            ),
            OwnershipLifecycleGuards::default(),
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::TerminalState
        ));
    }

    #[test]
    fn human_allows_sequential_lifecycle_progression_ces_b0_012_2() {
        let outcome = validate_human_transition(
            super::HumanTransitionRequest::new(
                HumanId::new("CX-EMP-000001").expect("human"),
                HumanLifecycle::Registration,
                HumanLifecycle::Active,
                None,
                None,
                None,
                sequence(),
            ),
            HumanLifecycleGuards {
                registration_completed: true,
                assignment_ready: true,
                ..HumanLifecycleGuards::default()
            },
        );
        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn human_rejects_non_adjacent_transition_ces_b0_012_2() {
        let outcome = validate_human_transition(
            super::HumanTransitionRequest::new(
                HumanId::new("CX-EMP-000001").expect("human"),
                HumanLifecycle::Candidate,
                HumanLifecycle::Training,
                None,
                None,
                None,
                sequence(),
            ),
            HumanLifecycleGuards::default(),
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::IllegalTransition
        ));
    }

    #[test]
    fn agent_requires_supervision_for_activation_ces_b0_027_7() {
        let outcome = validate_agent_transition(
            super::AgentTransitionRequest::new(
                AgentId::new("CX-AGT-000001").expect("agent"),
                AgentLifecycle::Approved,
                AgentLifecycle::Active,
                None,
                None,
                None,
                sequence(),
            ),
            AgentLifecycleGuards {
                valid_ownership: true,
                valid_permissions: true,
                current_supervision: false,
                trust_valid: true,
                lease_valid: true,
                ..AgentLifecycleGuards::default()
            },
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("agent activation requires ownership, permissions, supervision, trust, and lease validity")
        ));
    }

    #[test]
    fn agent_recovery_may_restore_to_registered_ces_b0_027_19() {
        let outcome = validate_agent_transition(
            super::AgentTransitionRequest::new(
                AgentId::new("CX-AGT-000001").expect("agent"),
                AgentLifecycle::Recovering,
                AgentLifecycle::Registered,
                Some(TransitionReasonReference::new("recovery completed").expect("reason")),
                None,
                Some(TransitionEvidenceReference::new("REC-001").expect("evidence")),
                sequence(),
            ),
            AgentLifecycleGuards {
                underlying_failure_resolved: true,
                current_supervision: true,
                trust_valid: true,
                ..AgentLifecycleGuards::default()
            },
        );
        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn decision_rejected_status_requires_rationale_and_authority_ces_b0_022_6() {
        let outcome = validate_decision_transition(
            super::DecisionTransitionRequest::new(
                DecisionId::new("CX-DEC-000001").expect("decision"),
                DecisionRecordStatus::PendingApproval,
                DecisionRecordStatus::Rejected,
                None,
                None,
                None,
                sequence(),
            ),
            &DecisionLifecycleGuards::default(),
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::MissingReason
        ));
    }

    #[test]
    fn decision_supersession_requires_successor_or_retirement_authority_ces_b0_022_5() {
        let outcome = validate_decision_transition(
            super::DecisionTransitionRequest::new(
                DecisionId::new("CX-DEC-000001").expect("decision"),
                DecisionRecordStatus::Approved,
                DecisionRecordStatus::Superseded,
                Some(TransitionReasonReference::new("replaced").expect("reason")),
                Some(TransitionAuthorityReference::new("authority").expect("authority")),
                Some(TransitionEvidenceReference::new("evidence").expect("evidence")),
                sequence(),
            ),
            &DecisionLifecycleGuards::default(),
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("decision supersession requires successor or retirement authority")
        ));
    }

    #[test]
    fn delegation_resume_requires_valid_upstream_state_ces_b0_029_13() {
        let outcome = validate_delegation_transition(
            super::DelegationTransitionRequest::new(
                DelegationId::new("CX-DEL-000001").expect("delegation"),
                DelegationLifecycle::Suspended,
                DelegationLifecycle::Active,
                None,
                None,
                None,
                sequence(),
            ),
            DelegationLifecycleGuards {
                policy_valid: true,
                authorization_valid: true,
                eligible_delegator: true,
                eligible_delegate: true,
                scope_compatible: true,
                separation_of_duties_satisfied: true,
                acceptance_required: false,
                acceptance_recorded: false,
                upstream_valid: false,
                expiration_rule_present: true,
                time_valid: true,
            },
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("delegation activation or resume requires valid policy, authorization, eligibility, scope, SoD, upstream state, time bounds, and expiration rule")
        ));
    }

    #[test]
    fn workflow_failure_transition_requires_stable_failure_code_ces_b0_030_13() {
        let outcome = validate_workflow_transition(
            super::WorkflowTransitionRequest::new(
                WorkflowId::new("CX-WF-000001").expect("workflow"),
                WorkflowState::Running,
                WorkflowState::Failed,
                None,
                None,
                None,
                sequence(),
            ),
            &WorkflowLifecycleGuards::default(),
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("workflow failure transition requires a stable failure code")
        ));
    }

    #[test]
    fn workflow_recovery_requires_revalidation_ces_b0_030_14() {
        let outcome = validate_workflow_transition(
            super::WorkflowTransitionRequest::new(
                WorkflowId::new("CX-WF-000001").expect("workflow"),
                WorkflowState::Failed,
                WorkflowState::Ready,
                None,
                None,
                Some(TransitionEvidenceReference::new("AUD-001").expect("evidence")),
                sequence(),
            ),
            &WorkflowLifecycleGuards {
                policy_valid: true,
                authorization_valid: true,
                delegation_valid: true,
                decision_valid: true,
                scope_valid: true,
                participants_valid: true,
                audit_evidence: Some(
                    TransitionEvidenceReference::new("AUD-001").expect("audit evidence"),
                ),
                upstream_outcomes_allow: true,
                retry_limit_respected: true,
                recovery_revalidated: false,
                failure_code: Some(WorkflowFailureCode::Timeout),
            },
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("workflow recovery to ready requires bounded retry and fresh revalidation")
        ));
    }

    #[test]
    fn workflow_repeated_validation_is_deterministic_ces_b0_030_17() {
        let request = super::WorkflowTransitionRequest::new(
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            WorkflowState::Ready,
            WorkflowState::Running,
            None,
            None,
            Some(TransitionEvidenceReference::new("AUD-002").expect("evidence")),
            sequence(),
        );
        let guards = WorkflowLifecycleGuards {
            policy_valid: true,
            authorization_valid: true,
            delegation_valid: true,
            decision_valid: true,
            scope_valid: true,
            participants_valid: true,
            audit_evidence: Some(
                TransitionEvidenceReference::new("AUD-002").expect("audit evidence"),
            ),
            upstream_outcomes_allow: true,
            retry_limit_respected: true,
            recovery_revalidated: true,
            failure_code: Some(WorkflowFailureCode::Timeout),
        };
        let first = validate_workflow_transition(request.clone(), &guards);
        let second = validate_workflow_transition(request, &guards);
        assert_eq!(first, second);
    }

    #[test]
    fn allowed_transition_record_preserves_subject_identity_ces_b0_029_20() {
        let outcome = validate_delegation_transition(
            super::DelegationTransitionRequest::new(
                DelegationId::new("CX-DEL-000777").expect("delegation"),
                DelegationLifecycle::Draft,
                DelegationLifecycle::Requested,
                None,
                None,
                None,
                sequence(),
            ),
            DelegationLifecycleGuards::default(),
        );
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };
        assert_eq!(allowed.subject_id().as_str(), "CX-DEL-000777");
    }

    #[test]
    fn workflow_failure_codes_are_stable_ces_b0_030_13() {
        assert_eq!(WorkflowFailureCode::PolicyDeny.as_str(), "WF_POLICY_DENY");
        assert_eq!(
            WorkflowFailureCode::AuthorizationDeny.as_str(),
            "WF_AUTHORIZATION_DENY"
        );
    }

    #[test]
    fn agent_state_snapshot_separates_identity_version_from_lifecycle_ces_b0_027_15() {
        let snapshot = super::AgentStateSnapshot::new(
            AgentId::new("CX-AGT-000001").expect("agent"),
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            crate::identifier::EnglishNamespace::new("agent_namespace", "enterprise.agent")
                .expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            AgentLifecycle::Suspended,
            sequence(),
        );
        assert_eq!(snapshot.agent_id().as_str(), "CX-AGT-000001");
        assert_eq!(snapshot.identity_version().as_str(), "1.0.0");
        assert_eq!(snapshot.lifecycle(), AgentLifecycle::Suspended);
    }

    #[test]
    fn workflow_state_snapshot_preserves_definition_version_ces_b0_030_17() {
        let snapshot = super::WorkflowStateSnapshot::new(
            WorkflowId::new("CX-WF-000001").expect("workflow"),
            OwnershipPath::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
                Some(ProjectId::new("CX-PROJ-000001").expect("project")),
                None,
            )
            .expect("path"),
            StableVersion::new("workflow_definition_version", "1.0.0").expect("version"),
            WorkflowState::Validated,
            sequence(),
        );
        assert_eq!(snapshot.definition_version().as_str(), "1.0.0");
        assert_eq!(snapshot.lifecycle(), WorkflowState::Validated);
    }

    #[test]
    fn decision_transition_allows_authorized_approval_ces_b0_022_5() {
        let outcome = validate_decision_transition(
            super::DecisionTransitionRequest::new(
                DecisionId::new("CX-DEC-000001").expect("decision"),
                DecisionRecordStatus::PendingApproval,
                DecisionRecordStatus::Approved,
                Some(TransitionReasonReference::new("validated").expect("reason")),
                Some(TransitionAuthorityReference::new("founder").expect("authority")),
                Some(TransitionEvidenceReference::new("approval-evidence").expect("evidence")),
                sequence(),
            ),
            &DecisionLifecycleGuards {
                validation_complete: true,
                approval_complete: true,
                successor_decision_id: None,
                retirement_authority_id: Some(
                    DecisionAuthorityId::new("CX-DECAUTH-000001").expect("authority"),
                ),
            },
        );
        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn project_completion_requires_success_criteria_ces_b0_025_3() {
        let outcome = validate_project_transition(
            super::ProjectTransitionRequest::new(
                ProjectId::new("CX-PROJ-000001").expect("project"),
                ProjectLifecycle::Active,
                ProjectLifecycle::Completed,
                None,
                None,
                None,
                sequence(),
            ),
            ProjectLifecycleGuards {
                success_criteria_met: false,
                ..ProjectLifecycleGuards::default()
            },
        );
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("project completion requires satisfied success criteria")
        ));
    }

    #[test]
    fn workflow_transition_control_legal_transition_returns_allowed_outcome() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Running,
        ));
        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn workflow_transition_control_illegal_transition_returns_rejected_outcome() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Draft,
            WorkflowState::Running,
        ));
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::IllegalTransition
        ));
    }

    #[test]
    fn workflow_transition_control_same_state_transition_returns_canonical_no_op_outcome() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Ready,
        ));
        assert!(matches!(outcome, TransitionOutcome::NoOp(_)));
    }

    #[test]
    fn workflow_transition_control_allowed_transition_advances_state_sequence_exactly_once() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Running,
        ));
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };

        assert_eq!(allowed.sequence().value(), 2);
    }

    #[test]
    fn workflow_transition_control_rejected_transition_does_not_advance_sequence() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Draft,
            WorkflowState::Running,
        ));
        let rejected = match outcome {
            TransitionOutcome::Rejected(rejected) => rejected,
            other => panic!("expected rejected outcome, got {other:?}"),
        };

        assert_eq!(rejected.sequence().value(), 1);
    }

    #[test]
    fn workflow_transition_control_no_op_transition_does_not_advance_sequence() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Ready,
        ));
        let noop = match outcome {
            TransitionOutcome::NoOp(noop) => noop,
            other => panic!("expected no-op outcome, got {other:?}"),
        };

        assert_eq!(noop.sequence().value(), 1);
    }

    #[test]
    fn workflow_transition_control_transition_reason_is_preserved() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Running,
        ));
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };

        assert_eq!(
            allowed.reason().expect("reason").as_str(),
            "operator request"
        );
    }

    #[test]
    fn workflow_transition_control_transition_authority_is_preserved() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Running,
        ));
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };

        assert_eq!(
            allowed.authority().expect("authority").as_str(),
            "authority.ref"
        );
    }

    #[test]
    fn workflow_transition_control_transition_evidence_order_is_preserved() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            None,
            None,
            vec![
                TransitionEvidenceReference::new("AUD-001").expect("evidence"),
                TransitionEvidenceReference::new("AUD-002").expect("evidence"),
            ],
            None,
            workflow_transition_guards(),
        )
        .expect("transition request");

        assert_eq!(
            request.transition_evidence_references()[0].as_str(),
            "AUD-001"
        );
        assert_eq!(
            request.transition_evidence_references()[1].as_str(),
            "AUD-002"
        );
    }

    #[test]
    fn workflow_transition_control_transition_evidence_does_not_populate_guard_audit_evidence() {
        let request = workflow_transition_request_without_guard_audit_evidence(
            WorkflowState::Ready,
            WorkflowState::Running,
        );
        let outcome = WorkflowTransitionControl::evaluate(&request);

        assert_eq!(request.workflow_lifecycle_guards().audit_evidence, None);
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::MissingEvidence
        ));
    }

    #[test]
    fn workflow_transition_control_duplicate_transition_evidence_is_rejected() {
        let error = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            None,
            None,
            vec![
                TransitionEvidenceReference::new("AUD-001").expect("evidence"),
                TransitionEvidenceReference::new("AUD-001").expect("evidence"),
            ],
            None,
            workflow_transition_guards(),
        )
        .expect_err("duplicate evidence must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowTransitionControl(
                "duplicate workflow transition evidence reference",
            )
        );
    }

    #[test]
    fn workflow_transition_control_failure_transition_requires_stable_failure_code() {
        let error = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Running),
            WorkflowState::Failed,
            None,
            None,
            vec![],
            None,
            workflow_transition_guards(),
        )
        .expect_err("missing failure code must fail");

        assert_eq!(
            error,
            crate::errors::DomainError::InvalidWorkflowTransitionControl(
                "workflow failure target requires stable failure code",
            )
        );
    }

    #[test]
    fn workflow_transition_control_missing_required_audit_evidence_remains_rejected() {
        let outcome = WorkflowTransitionControl::evaluate(
            &workflow_transition_request_without_guard_audit_evidence(
                WorkflowState::Ready,
                WorkflowState::Running,
            ),
        );

        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::MissingEvidence
        ));
    }

    #[test]
    fn workflow_transition_control_failure_code_is_preserved() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Running),
            WorkflowState::Failed,
            None,
            None,
            vec![],
            Some(WorkflowFailureCode::Timeout),
            workflow_transition_guards(),
        )
        .expect("transition request");
        let outcome = WorkflowTransitionControl::evaluate(&request);
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };

        assert_eq!(request.failure_code(), Some(WorkflowFailureCode::Timeout));
        assert_eq!(*allowed.to(), WorkflowState::Failed);
    }

    #[test]
    fn workflow_transition_control_explicitly_supplied_guard_audit_evidence_is_preserved() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            None,
            None,
            vec![TransitionEvidenceReference::new("TRANS-001").expect("evidence")],
            None,
            WorkflowLifecycleGuards {
                audit_evidence: Some(
                    TransitionEvidenceReference::new("GUARD-001").expect("audit evidence"),
                ),
                ..workflow_transition_guards()
            },
        )
        .expect("transition request");
        let before = request.clone();
        let outcome = WorkflowTransitionControl::evaluate(&request);
        let allowed = match outcome {
            TransitionOutcome::Allowed(allowed) => allowed,
            other => panic!("expected allowed outcome, got {other:?}"),
        };

        assert_eq!(
            before
                .workflow_lifecycle_guards()
                .audit_evidence
                .as_ref()
                .expect("guard audit evidence")
                .as_str(),
            "GUARD-001"
        );
        assert_eq!(
            request
                .workflow_lifecycle_guards()
                .audit_evidence
                .as_ref()
                .expect("guard audit evidence")
                .as_str(),
            "GUARD-001"
        );
        assert_eq!(
            allowed.evidence().expect("transition evidence").as_str(),
            "TRANS-001"
        );
    }

    #[test]
    fn workflow_transition_control_illegal_terminal_state_transition_is_rejected() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Archived,
            WorkflowState::Ready,
        ));
        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason() == &TransitionRejectionReason::TerminalState
        ));
    }

    #[test]
    fn workflow_transition_control_equivalent_requests_return_equivalent_outcomes() {
        let request = workflow_transition_request(WorkflowState::Ready, WorkflowState::Running);
        let first = WorkflowTransitionControl::evaluate(&request);
        let second = WorkflowTransitionControl::evaluate(&request);

        assert_eq!(first, second);
    }

    #[test]
    fn workflow_transition_control_construction_and_evaluation_are_deterministic() {
        let first = workflow_transition_request(WorkflowState::Ready, WorkflowState::Running);
        let second = workflow_transition_request(WorkflowState::Ready, WorkflowState::Running);

        assert_eq!(first, second);
        assert_eq!(
            WorkflowTransitionControl::evaluate(&first),
            WorkflowTransitionControl::evaluate(&second)
        );
    }

    #[test]
    fn workflow_transition_control_supplied_request_values_are_not_mutated() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            Some(TransitionReasonReference::new("operator request").expect("reason")),
            Some(TransitionAuthorityReference::new("authority.ref").expect("authority")),
            vec![
                TransitionEvidenceReference::new("AUD-001").expect("evidence"),
                TransitionEvidenceReference::new("AUD-002").expect("evidence"),
            ],
            None,
            workflow_transition_guards(),
        )
        .expect("transition request");
        let before = request.clone();

        let _ = WorkflowTransitionControl::evaluate(&request);

        assert_eq!(request, before);
    }

    #[test]
    fn workflow_transition_control_external_authority_existence_is_not_checked() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            None,
            Some(TransitionAuthorityReference::new("external.authority").expect("authority")),
            vec![TransitionEvidenceReference::new("AUD-001").expect("evidence")],
            None,
            workflow_transition_guards(),
        )
        .expect("transition request");
        let outcome = WorkflowTransitionControl::evaluate(&request);

        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn workflow_transition_control_external_evidence_existence_is_not_checked() {
        let request = WorkflowTransitionControlRequest::new(
            workflow_transition_snapshot(WorkflowState::Ready),
            WorkflowState::Running,
            None,
            None,
            vec![TransitionEvidenceReference::new("AUD-999").expect("evidence")],
            None,
            WorkflowLifecycleGuards {
                audit_evidence: Some(
                    TransitionEvidenceReference::new("AUD-999").expect("audit evidence"),
                ),
                ..workflow_transition_guards()
            },
        )
        .expect("transition request");
        let outcome = WorkflowTransitionControl::evaluate(&request);

        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn workflow_transition_control_no_event_is_emitted() {
        let outcome = WorkflowTransitionControl::evaluate(&workflow_transition_request(
            WorkflowState::Ready,
            WorkflowState::Running,
        ));

        assert!(matches!(outcome, TransitionOutcome::Allowed(_)));
    }

    #[test]
    fn workflow_transition_control_no_workflow_instance_is_mutated() {
        let instance = workflow_instance(WorkflowState::Ready);
        let before = instance.clone();
        let request = WorkflowTransitionControlRequest::new(
            instance.current_workflow_state_snapshot().clone(),
            WorkflowState::Running,
            None,
            None,
            vec![TransitionEvidenceReference::new("AUD-001").expect("evidence")],
            None,
            workflow_transition_guards(),
        )
        .expect("transition request");

        let _ = WorkflowTransitionControl::evaluate(&request);

        assert_eq!(instance, before);
    }

    #[test]
    fn workflow_transition_control_existing_k2_lifecycle_behavior_remains_unchanged() {
        let outcome = validate_workflow_transition(
            super::WorkflowTransitionRequest::new(
                WorkflowId::new("CX-WF-000001").expect("workflow"),
                WorkflowState::Running,
                WorkflowState::Failed,
                None,
                None,
                None,
                sequence(),
            ),
            &WorkflowLifecycleGuards::default(),
        );

        assert!(matches!(
            outcome,
            TransitionOutcome::Rejected(rejection)
                if rejection.reason()
                    == &TransitionRejectionReason::GuardFailed("workflow failure transition requires a stable failure code")
        ));
    }

    #[test]
    fn workflow_transition_control_existing_workflow_definition_and_instance_apis_remain_usable() {
        let definition = workflow_definition();
        let instance = workflow_instance(WorkflowState::Ready);

        assert_eq!(definition.workflow_id().as_str(), "CX-WF-000001");
        assert_eq!(instance.workflow_definition(), &definition);
    }
}
