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
pub mod request;

pub use agent::{
    AgentCategory, AgentDefinition, AgentReference, AgentRuntimeReference, AgentType,
};
pub use authorization::{
    ActionVerb, AuthorityLevel, AuthorizationDecisionOutcome, AuthorizationDecisionReference,
    AuthorizationPrincipalReference, AuthorizationPrincipalType, AuthorizationSubject,
    AuthorizationTarget, CredentialStatusReference, PermissionEffectIntent, PermissionReference,
    PrincipalLifecycleStateReference, ResourceType, RoleReference, ScopeLevel, ScopeReference,
};
pub use decision::{
    DecisionContextReference, DecisionOutcome, DecisionOwnerReference, DecisionPolicySetReference,
    DecisionRationaleReference, DecisionRecord, DecisionStatus, DecisionSubjectReference,
    DecisionType,
};
pub use delegation::{
    AuthoritySourceReference, DelegatedRightReference, DelegatedTaskReference,
    DelegationConditionReference, DelegationDepth, DelegationReference, DelegationScope,
    DelegationScopeKind, DelegationVersion, DelegateReference, DelegatorReference,
    BeneficiaryReference, PolicyResultReference, SeparationOfDutiesConflict,
};
pub use errors::{DomainError, DomainResult};
pub use identifier::{
    AgentId, AgentUuid, AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId,
    DecisionAuthorityId, DecisionId, DelegationId, EnglishNamespace, EnterpriseId, HumanId,
    NonEmptyText, OrganizationUnitId, OwnershipId, PermissionId, PolicyId, PrincipalId,
    ProjectId, RoleId, ScopeId, StableVersion, WorkflowId, WorkspaceId,
};
pub use identity::{IdentityKind, IdentityReference};
pub use identity::{AgentIdentity, HumanIdentity};
pub use lifecycle::{
    AgentLifecycle, DecisionRecordStatus, DelegationLifecycle, EnterpriseLifecycle, HumanLifecycle,
    OrganizationalUnitLifecycle, OwnershipLifecycle, ProjectLifecycle, WorkflowState,
    WorkspaceLifecycle,
};
pub use ownership::{OrganizationalContext, OwnershipPath, OwnershipScope, OwnershipSubject, OwnerReference};
pub use request::{AuthorizationRequestRecord, TimeReference};
