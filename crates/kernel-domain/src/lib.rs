#![forbid(unsafe_code)]

pub mod agent;
pub mod authorization;
pub mod decision;
pub mod delegation;
pub mod errors;
pub mod identifier;
pub mod identity;
pub mod lifecycle;
pub mod ownership;
pub mod policy;
pub mod request;
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
pub use errors::{DomainError, DomainResult};
pub use identifier::{
    AgentId, AgentUuid, AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId,
    DecisionAuthorityId, DecisionId, DelegationId, EnglishNamespace, EnterpriseId, HumanId,
    NonEmptyText, OrganizationUnitId, OwnershipId, PermissionId, PolicyId, PrincipalId, ProjectId,
    RoleId, ScopeId, StableVersion, WorkflowId, WorkspaceId,
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
