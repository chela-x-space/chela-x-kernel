#![forbid(unsafe_code)]

pub mod agent;
pub mod authorization;
pub mod decision;
pub mod delegation;
pub mod enforcement;
pub mod errors;
pub mod event;
pub mod identifier;
pub mod identity;
pub mod lifecycle;
pub mod ownership;
pub mod policy;
pub mod request;
pub mod runtime;
pub mod state;
pub mod workflow;

pub use agent::{
    AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentFailureCategory,
    AgentFailureReference, AgentFailureSeverity, AgentRecoveryEvidenceReference,
    AgentRecoveryPlanReference, AgentRecoveryReference, AgentReference, AgentRuntimeReference,
    AgentType,
};
pub use authorization::{
    ActionVerb, AuthorityLevel, AuthorizationAuditEvidenceReference, AuthorizationDecisionOutcome,
    AuthorizationDecisionReference, AuthorizationEvaluationOrderVersion,
    AuthorizationEvaluationStep, AuthorizationPrincipalReference, AuthorizationPrincipalType,
    AuthorizationSubject, AuthorizationTarget, CredentialStatusReference,
    MatchedPolicyEvidenceReference, PermissionEffectIntent, PermissionReference,
    PrincipalLifecycleStateReference, ResourceType, RoleReference, ScopeLevel, ScopeReference,
};
pub use decision::{
    DecisionContextReference, DecisionOutcome, DecisionOwnerReference, DecisionPolicySetReference,
    DecisionRationaleReference, DecisionRecord, DecisionRecordSpec, DecisionStatus,
    DecisionSubjectReference, DecisionType,
};
pub use delegation::{
    AuthoritySourceReference, BeneficiaryReference, DelegateReference, DelegatedRightReference,
    DelegatedTaskReference, DelegationConditionReference, DelegationDepth, DelegationReference,
    DelegationReferenceSpec, DelegationScope, DelegationScopeKind, DelegationVersion,
    DelegatorReference, PolicyResultReference, SeparationOfDutiesConflict,
};
pub use enforcement::{
    evaluate_authorization, AuthorityValidationResult, AuthorizationAuthorityRequirement,
    AuthorizationDecisionIds, AuthorizationDelegationBinding, AuthorizationEvaluationContext,
    AuthorizationEvaluationInput, AuthorizationEvaluationResult, AuthorizationEvaluationStepResult,
    AuthorizationEvaluationTrace, AuthorizationExplicitDenyRecord, AuthorizationGrantRecord,
    AuthorizationGrantRecordSpec, AuthorizationPolicyLayer, AuthorizationPolicyRecord,
    AuthorizationPolicyRecordSpec, AuthorizationRejectionReason, AuthorizationRolePermissionRecord,
    DecisionConstructionInput, DelegationBoundResult, PermissionMatchResult, PolicyMatchResult,
    ScopeValidationResult, SeparationOfDutiesResult,
};
pub use errors::{DomainError, DomainResult};
pub use event::{
    validate_event_envelope, validate_event_identity, validate_event_timestamps,
    validate_event_version, EventActorId, EventCausation, EventClassification, EventComponent,
    EventEnvelope, EventEnvelopeCandidate, EventSource, EventSubject, EventSubjectId,
    EventSubjectType, EventTrace, EventTraceReference, EventType, EventVersion,
};
pub use identifier::{
    AgentId, AgentUuid, AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId,
    CapabilityId, CorrelationId, DecisionAuthorityId, DecisionId, DelegationId, EnglishNamespace,
    EnterpriseId, EventId, HeartbeatId, HumanId, LeaseId, NonEmptyText, OrganizationUnitId,
    OwnershipId, PermissionId, PolicyId, PrincipalId, ProjectId, RoleId, RuntimeId, ScopeId,
    StableVersion, WorkflowId, WorkspaceId,
};
pub use identity::{AgentIdentity, HumanIdentity};
pub use identity::{IdentityKind, IdentityReference};
pub use lifecycle::{
    AgentLifecycle, DecisionRecordStatus, DelegationLifecycle, EnterpriseLifecycle, HumanLifecycle,
    OrganizationalUnitLifecycle, OwnershipLifecycle, ProjectLifecycle, WorkflowState,
    WorkspaceLifecycle,
};
pub use ownership::{
    OrganizationalContext, OwnerReference, OwnershipPath, OwnershipScope, OwnershipSubject,
};
pub use policy::{
    PolicyAuditEvidenceReference, PolicyEffect, PolicyEvaluationOrderVersion, PolicyEvaluationStep,
};
pub use request::{AuthorizationRequestRecord, TimeReference};
pub use runtime::{
    assess_heartbeat, assess_recovery_eligibility, assess_runtime_health, evaluate_lease,
    supervise_runtime, AgentRegistration, AgentRegistrationSpec, AgentRegistry,
    CapabilityDescriptor, CapabilityDescriptorSpec, HeartbeatAssessment, HeartbeatFreshness,
    HeartbeatFreshnessPolicy, HeartbeatObservationSpec, HeartbeatRecord, HeartbeatRecordOutcome,
    HeartbeatRecordSpec, HeartbeatUpdateResult, LeaseAssessment, LeaseIssuanceSpec, LeasePolicy,
    LeaseRecord, LeaseRecordSpec, LeaseRejectionReason, LeaseRenewalOutcome, LeaseRenewalRequest,
    LeaseRenewalResult, LeaseValidity, PresenceState, RecoveryEligibility, RecoveryRejectionReason,
    RuntimeEntity, RuntimeEntitySpec, RuntimeFailureObservation, RuntimeHealth,
    RuntimeStateSnapshot, SupervisorAction, SupervisorEvidenceReference, SupervisorObservation,
    SupervisorOutcome, SupervisorStep, SupervisorTrace, SupervisorTraceStep,
};
pub use state::{
    validate_agent_transition, validate_decision_transition, validate_delegation_transition,
    validate_enterprise_transition, validate_human_transition,
    validate_organizational_unit_transition, validate_ownership_transition,
    validate_project_transition, validate_workflow_transition, validate_workspace_transition,
    AgentLifecycleGuards, AgentStateSnapshot, AgentTransitionOutcome, AgentTransitionRequest,
    AgentTransitionStateSnapshot, AllowedTransition, DecisionLifecycleGuards,
    DecisionStateSnapshot, DecisionTransitionOutcome, DecisionTransitionRequest,
    DelegationLifecycleGuards, DelegationStateSnapshot, DelegationTransitionOutcome,
    DelegationTransitionRequest, EnterpriseLifecycleGuards, EnterpriseStateSnapshot,
    EnterpriseTransitionOutcome, EnterpriseTransitionRequest, HumanLifecycleGuards,
    HumanStateSnapshot, HumanTransitionOutcome, HumanTransitionRequest, NoOpTransition,
    OrganizationalUnitLifecycleGuards, OrganizationalUnitStateSnapshot,
    OrganizationalUnitTransitionOutcome, OrganizationalUnitTransitionRequest,
    OwnershipLifecycleGuards, OwnershipStateSnapshot, OwnershipTransitionOutcome,
    OwnershipTransitionRequest, ProjectLifecycleGuards, ProjectStateSnapshot,
    ProjectTransitionOutcome, ProjectTransitionRequest, RejectedTransition, StateSequence,
    StateSnapshot, TransitionAuthorityReference, TransitionEvidenceReference, TransitionOutcome,
    TransitionReasonReference, TransitionRejectionReason, WorkflowFailureCode,
    WorkflowLifecycleGuards, WorkflowStateSnapshot, WorkflowTransitionOutcome,
    WorkflowTransitionRequest, WorkflowTransitionStateSnapshot, WorkspaceLifecycleGuards,
    WorkspaceStateSnapshot, WorkspaceTransitionOutcome, WorkspaceTransitionRequest,
};
pub use workflow::{
    WorkflowAuditEvidenceReference, WorkflowRecoveryReference, WorkflowRetryLimit,
    WorkflowRetryPolicyReference,
};
