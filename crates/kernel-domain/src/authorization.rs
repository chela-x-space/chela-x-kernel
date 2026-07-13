use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    AuthorizationDecisionId, EnterpriseId, NonEmptyText, PermissionId, PolicyId, PrincipalId,
    RoleId, ScopeId,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialStatusReference(NonEmptyText);

impl CredentialStatusReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("credential_status", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPrincipalReference {
    principal_id: PrincipalId,
    principal_type: AuthorizationPrincipalType,
    bound_identity_id: String,
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
            bound_identity_id: NonEmptyText::new("bound_identity_id", bound_identity_id)?.to_string(),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleReference {
    role_id: RoleId,
    enterprise_id: EnterpriseId,
}

impl RoleReference {
    pub fn new(role_id: RoleId, enterprise_id: EnterpriseId) -> Self {
        Self { role_id, enterprise_id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionVerb(NonEmptyText);

impl ActionVerb {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("action_verb", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceType(NonEmptyText);

impl ResourceType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("resource_type", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermissionEffectIntent(NonEmptyText);

impl PermissionEffectIntent {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("permission_effect_intent", value)?))
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
        let has_resource = resource_id.as_ref().map(|value| !value.trim().is_empty()).unwrap_or(false);
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
            ScopeLevel::OrganizationalUnit if ownership_path.organizational_unit_id().is_none() || has_resource => {
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

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationDecisionReference {
    decision_id: AuthorizationDecisionId,
    policy_id: PolicyId,
    outcome: AuthorizationDecisionOutcome,
}

impl AuthorizationDecisionReference {
    pub fn new(
        decision_id: AuthorizationDecisionId,
        policy_id: PolicyId,
        outcome: AuthorizationDecisionOutcome,
    ) -> Self {
        Self {
            decision_id,
            policy_id,
            outcome,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ActionVerb, AuthorizationPrincipalReference, AuthorizationPrincipalType, AuthorizationSubject,
        AuthorizationTarget, CredentialStatusReference, PermissionEffectIntent, PermissionReference,
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
        assert!(error.to_string().contains("project scope requires a project"));
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
        assert_eq!(AuthorizationSubject::Principal(principal).enterprise_id(), target.scope().enterprise_id());
    }
}
