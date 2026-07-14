use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuditEvidenceId, AuthorizationDecisionId, AuthorizationRequestId, EnterpriseId, NonEmptyText,
    PermissionId, PolicyId, PrincipalId, RoleId, ScopeId, StableVersion,
};
use crate::ownership::OwnershipPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityLevel {
    Observer,
    Operator,
    Specialist,
    Manager,
    Director,
    Executive,
    Founder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationPrincipalType {
    Employee,
    Service,
    Workflow,
    Runtime,
    DelegatedAgent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalLifecycleStateReference(NonEmptyText);

impl PrincipalLifecycleStateReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("principal_lifecycle_state", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialStatusReference(NonEmptyText);

impl CredentialStatusReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("credential_status", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPrincipalReference {
    principal_id: PrincipalId,
    principal_type: AuthorizationPrincipalType,
    bound_identity_id: NonEmptyText,
    enterprise_id: EnterpriseId,
    lifecycle_state: PrincipalLifecycleStateReference,
    credential_status: CredentialStatusReference,
}

impl AuthorizationPrincipalReference {
    pub fn new(
        principal_id: PrincipalId,
        principal_type: AuthorizationPrincipalType,
        bound_identity_id: impl Into<String>,
        enterprise_id: EnterpriseId,
        lifecycle_state: PrincipalLifecycleStateReference,
        credential_status: CredentialStatusReference,
    ) -> DomainResult<Self> {
        Ok(Self {
            principal_id,
            principal_type,
            bound_identity_id: NonEmptyText::new("bound_identity_id", bound_identity_id)?,
            enterprise_id,
            lifecycle_state,
            credential_status,
        })
    }

    pub fn principal_id(&self) -> &PrincipalId {
        &self.principal_id
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn principal_type(&self) -> AuthorizationPrincipalType {
        self.principal_type
    }

    pub fn bound_identity_id(&self) -> &str {
        self.bound_identity_id.as_str()
    }

    pub fn lifecycle_state(&self) -> &PrincipalLifecycleStateReference {
        &self.lifecycle_state
    }

    pub fn credential_status(&self) -> &CredentialStatusReference {
        &self.credential_status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleReference {
    role_id: RoleId,
    enterprise_id: EnterpriseId,
}

impl RoleReference {
    pub fn new(role_id: RoleId, enterprise_id: EnterpriseId) -> Self {
        Self {
            role_id,
            enterprise_id,
        }
    }

    pub fn role_id(&self) -> &RoleId {
        &self.role_id
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionVerb(NonEmptyText);

impl ActionVerb {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("action_verb", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceType(NonEmptyText);

impl ResourceType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("resource_type", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionEffectIntent(NonEmptyText);

impl PermissionEffectIntent {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("permission_effect_intent", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionReference {
    permission_id: PermissionId,
    action_verb: ActionVerb,
    resource_type: ResourceType,
    effect_intent: PermissionEffectIntent,
}

impl PermissionReference {
    pub fn new(
        permission_id: PermissionId,
        action_verb: ActionVerb,
        resource_type: ResourceType,
        effect_intent: PermissionEffectIntent,
    ) -> Self {
        Self {
            permission_id,
            action_verb,
            resource_type,
            effect_intent,
        }
    }

    pub fn permission_id(&self) -> &PermissionId {
        &self.permission_id
    }

    pub fn action_verb(&self) -> &ActionVerb {
        &self.action_verb
    }

    pub fn resource_type(&self) -> &ResourceType {
        &self.resource_type
    }

    pub fn effect_intent(&self) -> &PermissionEffectIntent {
        &self.effect_intent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeLevel {
    Enterprise,
    Workspace,
    Project,
    OrganizationalUnit,
    Resource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeReference {
    scope_id: ScopeId,
    level: ScopeLevel,
    ownership_path: OwnershipPath,
    resource_id: Option<String>,
}

impl ScopeReference {
    pub fn new(
        scope_id: ScopeId,
        level: ScopeLevel,
        ownership_path: OwnershipPath,
        resource_id: Option<String>,
    ) -> DomainResult<Self> {
        let has_workspace = ownership_path.workspace_id().is_some();
        let has_project = ownership_path.project_id().is_some();
        let has_resource = resource_id
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        match level {
            ScopeLevel::Enterprise if has_workspace || has_project || has_resource => {
                return Err(DomainError::InvalidAuthorizationReference(
                    "enterprise scope cannot carry workspace, project, or resource identifiers",
                ));
            }
            ScopeLevel::Workspace if !has_workspace || has_project || has_resource => {
                return Err(DomainError::InvalidAuthorizationReference(
                    "workspace scope requires workspace and forbids project or resource identifiers",
                ));
            }
            ScopeLevel::Project if !has_project || has_resource => {
                return Err(DomainError::InvalidAuthorizationReference(
                    "project scope requires a project and forbids resource identifiers",
                ));
            }
            ScopeLevel::OrganizationalUnit
                if ownership_path.organizational_unit_id().is_none() || has_resource =>
            {
                return Err(DomainError::InvalidAuthorizationReference(
                    "organizational unit scope requires an organizational unit and forbids resource identifiers",
                ));
            }
            ScopeLevel::Resource if !has_resource => {
                return Err(DomainError::InvalidAuthorizationReference(
                    "resource scope requires a non-empty resource identifier",
                ));
            }
            _ => {}
        }
        Ok(Self {
            scope_id,
            level,
            ownership_path,
            resource_id,
        })
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        self.ownership_path.enterprise_id()
    }

    pub fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    pub fn level(&self) -> ScopeLevel {
        self.level
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationSubject {
    Principal(AuthorizationPrincipalReference),
}

impl AuthorizationSubject {
    pub fn enterprise_id(&self) -> &EnterpriseId {
        match self {
            Self::Principal(principal) => principal.enterprise_id(),
        }
    }

    pub fn principal(&self) -> &AuthorizationPrincipalReference {
        match self {
            Self::Principal(principal) => principal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationTarget {
    resource_type: ResourceType,
    resource_identifier: NonEmptyText,
    scope: ScopeReference,
}

impl AuthorizationTarget {
    pub fn new(
        resource_type: ResourceType,
        resource_identifier: impl Into<String>,
        scope: ScopeReference,
    ) -> DomainResult<Self> {
        Ok(Self {
            resource_type,
            resource_identifier: NonEmptyText::new("resource_identifier", resource_identifier)?,
            scope,
        })
    }

    pub fn scope(&self) -> &ScopeReference {
        &self.scope
    }

    pub fn resource_type(&self) -> &ResourceType {
        &self.resource_type
    }

    pub fn resource_identifier(&self) -> &str {
        self.resource_identifier.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationDecisionOutcome {
    Allow,
    Deny,
    DenyExplicit,
    DenyScope,
    DenySeparationOfDuties,
    DenyValidation,
}

impl AuthorizationDecisionOutcome {
    pub fn is_denied(self) -> bool {
        !matches!(self, Self::Allow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationEvaluationStep {
    VerifyPrincipalIdentityAndLifecycle,
    VerifyTenantIsolationAndScopeLineage,
    ResolveExplicitDenials,
    ResolveDirectGrants,
    ResolveInheritedGrants,
    ResolveRequestedPermissionMatch,
    ApplySeparationOfDutiesConflicts,
    EmitFinalDecisionAndEvidence,
}

impl AuthorizationEvaluationStep {
    pub fn ordered() -> [Self; 8] {
        [
            Self::VerifyPrincipalIdentityAndLifecycle,
            Self::VerifyTenantIsolationAndScopeLineage,
            Self::ResolveExplicitDenials,
            Self::ResolveDirectGrants,
            Self::ResolveInheritedGrants,
            Self::ResolveRequestedPermissionMatch,
            Self::ApplySeparationOfDutiesConflicts,
            Self::EmitFinalDecisionAndEvidence,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationEvaluationOrderVersion(StableVersion);

impl AuthorizationEvaluationOrderVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(StableVersion::new(
            "authorization_evaluation_order_version",
            value,
        )?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchedPolicyEvidenceReference(NonEmptyText);

impl MatchedPolicyEvidenceReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("matched_policy_evidence", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDecisionReference {
    decision_id: AuthorizationDecisionId,
    request_id: AuthorizationRequestId,
    policy_id: PolicyId,
    outcome: AuthorizationDecisionOutcome,
    evaluation_order_version: AuthorizationEvaluationOrderVersion,
    matched_policy_evidence: MatchedPolicyEvidenceReference,
    decided_at: NonEmptyText,
}

impl AuthorizationDecisionReference {
    pub fn new(
        decision_id: AuthorizationDecisionId,
        request_id: AuthorizationRequestId,
        policy_id: PolicyId,
        outcome: AuthorizationDecisionOutcome,
        evaluation_order_version: AuthorizationEvaluationOrderVersion,
        matched_policy_evidence: MatchedPolicyEvidenceReference,
        decided_at: impl Into<String>,
    ) -> DomainResult<Self> {
        Ok(Self {
            decision_id,
            request_id,
            policy_id,
            outcome,
            evaluation_order_version,
            matched_policy_evidence,
            decided_at: NonEmptyText::new("authorization_decided_at", decided_at)?,
        })
    }

    pub fn decision_id(&self) -> &AuthorizationDecisionId {
        &self.decision_id
    }

    pub fn request_id(&self) -> &AuthorizationRequestId {
        &self.request_id
    }

    pub fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    pub fn outcome(&self) -> AuthorizationDecisionOutcome {
        self.outcome
    }

    pub fn evaluation_order_version(&self) -> &AuthorizationEvaluationOrderVersion {
        &self.evaluation_order_version
    }

    pub fn matched_policy_evidence(&self) -> &MatchedPolicyEvidenceReference {
        &self.matched_policy_evidence
    }

    pub fn decided_at(&self) -> &str {
        self.decided_at.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationAuditEvidenceReference {
    audit_evidence_id: AuditEvidenceId,
    decision_id: AuthorizationDecisionId,
    principal_id: PrincipalId,
    scope_id: ScopeId,
    policy_version: StableVersion,
    matched_rules: Vec<MatchedPolicyEvidenceReference>,
    outcome: AuthorizationDecisionOutcome,
}

impl AuthorizationAuditEvidenceReference {
    pub fn new(
        audit_evidence_id: AuditEvidenceId,
        decision_id: AuthorizationDecisionId,
        principal_id: PrincipalId,
        scope_id: ScopeId,
        policy_version: StableVersion,
        matched_rules: Vec<MatchedPolicyEvidenceReference>,
        outcome: AuthorizationDecisionOutcome,
    ) -> DomainResult<Self> {
        if matched_rules.is_empty() {
            return Err(DomainError::InvalidAuthorizationReference(
                "authorization audit evidence requires matched rule references",
            ));
        }
        Ok(Self {
            audit_evidence_id,
            decision_id,
            principal_id,
            scope_id,
            policy_version,
            matched_rules,
            outcome,
        })
    }

    pub fn audit_evidence_id(&self) -> &AuditEvidenceId {
        &self.audit_evidence_id
    }

    pub fn decision_id(&self) -> &AuthorizationDecisionId {
        &self.decision_id
    }

    pub fn principal_id(&self) -> &PrincipalId {
        &self.principal_id
    }

    pub fn scope_id(&self) -> &ScopeId {
        &self.scope_id
    }

    pub fn policy_version(&self) -> &StableVersion {
        &self.policy_version
    }

    pub fn matched_rules(&self) -> &[MatchedPolicyEvidenceReference] {
        &self.matched_rules
    }

    pub fn outcome(&self) -> AuthorizationDecisionOutcome {
        self.outcome
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ActionVerb, AuthorizationEvaluationStep, AuthorizationPrincipalReference,
        AuthorizationPrincipalType, AuthorizationSubject, AuthorizationTarget,
        CredentialStatusReference, PermissionEffectIntent, PermissionReference,
        PrincipalLifecycleStateReference, ResourceType, ScopeLevel, ScopeReference,
    };
    use crate::identifier::{EnterpriseId, PermissionId, PrincipalId, ScopeId};
    use crate::ownership::OwnershipPath;

    #[test]
    fn authorization_scope_accepts_valid_project_scope_ces_b0_026_3() {
        let scope = ScopeReference::new(
            ScopeId::new("CX-SCP-000001").expect("scope"),
            ScopeLevel::Project,
            OwnershipPath::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                Some(crate::identifier::WorkspaceId::new("CX-WS-000001").expect("workspace")),
                Some(crate::identifier::ProjectId::new("CX-PROJ-000001").expect("project")),
                None,
            )
            .expect("path"),
            None,
        )
        .expect("project scope");
        assert_eq!(scope.enterprise_id().as_str(), "CX-ENT-000001");
    }

    #[test]
    fn authorization_scope_rejects_invalid_target_reference_ces_b0_026_3() {
        let error = ScopeReference::new(
            ScopeId::new("CX-SCP-000001").expect("scope"),
            ScopeLevel::Project,
            OwnershipPath::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                None,
                None,
                None,
            )
            .expect("path"),
            None,
        )
        .expect_err("project scope without project must fail");
        assert!(error
            .to_string()
            .contains("project scope requires a project"));
    }

    #[test]
    fn authorization_request_principal_and_target_share_enterprise_traceability_k1() {
        let principal = AuthorizationPrincipalReference::new(
            PrincipalId::new("CX-PRN-000001").expect("principal"),
            AuthorizationPrincipalType::Employee,
            "CX-EMP-000001",
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            PrincipalLifecycleStateReference::new("Active").expect("lifecycle"),
            CredentialStatusReference::new("Valid").expect("credential"),
        )
        .expect("principal reference");
        let target = AuthorizationTarget::new(
            ResourceType::new("workflow").expect("resource type"),
            "workflow-001",
            ScopeReference::new(
                ScopeId::new("CX-SCP-000001").expect("scope"),
                ScopeLevel::Workspace,
                OwnershipPath::new(
                    EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                    Some(crate::identifier::WorkspaceId::new("CX-WS-000001").expect("workspace")),
                    None,
                    None,
                )
                .expect("path"),
                None,
            )
            .expect("scope"),
        )
        .expect("target");
        let _permission = PermissionReference::new(
            PermissionId::new("CX-PERM-000001").expect("permission"),
            ActionVerb::new("approve").expect("verb"),
            ResourceType::new("workflow").expect("resource type"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        );
        assert_eq!(
            AuthorizationSubject::Principal(principal).enterprise_id(),
            target.scope().enterprise_id()
        );
    }

    #[test]
    fn authorization_evaluation_order_matches_ces_b0_026_5() {
        let steps = AuthorizationEvaluationStep::ordered();
        assert_eq!(steps.len(), 8);
        assert_eq!(
            steps[0],
            AuthorizationEvaluationStep::VerifyPrincipalIdentityAndLifecycle
        );
        assert_eq!(
            steps[7],
            AuthorizationEvaluationStep::EmitFinalDecisionAndEvidence
        );
    }
}
