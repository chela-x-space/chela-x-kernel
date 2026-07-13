use crate::agent::AgentReference;
use crate::authorization::{
    AuthorizationDecisionOutcome, AuthorizationDecisionReference, PermissionReference,
};
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{
    DelegationId, EnglishNamespace, NonEmptyText, PolicyId, StableVersion,
};
use crate::lifecycle::DelegationLifecycle;
use crate::ownership::OwnershipPath;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationVersion(StableVersion);

impl DelegationVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(StableVersion::new("delegation_version", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatorReference {
    agent: AgentReference,
}

impl DelegatorReference {
    pub fn new(agent: AgentReference) -> Self {
        Self { agent }
    }

    pub fn agent_id(&self) -> &crate::identifier::AgentId {
        self.agent.agent_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegateReference {
    agent: AgentReference,
}

impl DelegateReference {
    pub fn new(agent: AgentReference) -> Self {
        Self { agent }
    }

    pub fn agent_id(&self) -> &crate::identifier::AgentId {
        self.agent.agent_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeneficiaryReference {
    Delegate(DelegateReference),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegationScopeKind {
    Enterprise,
    Workspace,
    Project,
    OrganizationalUnit,
    Agent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationScope {
    kind: DelegationScopeKind,
    ownership_path: OwnershipPath,
    agent_scope: Option<crate::identifier::AgentId>,
}

impl DelegationScope {
    pub fn new(
        kind: DelegationScopeKind,
        ownership_path: OwnershipPath,
        agent_scope: Option<crate::identifier::AgentId>,
    ) -> DomainResult<Self> {
        if matches!(kind, DelegationScopeKind::Agent) && agent_scope.is_none() {
            return Err(DomainError::InvalidDelegationReference(
                "agent scope requires an agent identifier",
            ));
        }
        Ok(Self {
            kind,
            ownership_path,
            agent_scope,
        })
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedTaskReference(NonEmptyText);

impl DelegatedTaskReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("delegated_task", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedRightReference {
    permission: PermissionReference,
    responsibility: NonEmptyText,
}

impl DelegatedRightReference {
    pub fn new(permission: PermissionReference, responsibility: impl Into<String>) -> DomainResult<Self> {
        Ok(Self {
            permission,
            responsibility: NonEmptyText::new("delegated_responsibility", responsibility)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationConditionReference(NonEmptyText);

impl DelegationConditionReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("delegation_condition", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationDepth(u16);

impl DelegationDepth {
    pub fn new(value: i64) -> DomainResult<Self> {
        if value < 0 {
            return Err(DomainError::InvalidDelegationReference(
                "delegation depth cannot be negative",
            ));
        }
        Ok(Self(value as u16))
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResultReference {
    policy_id: PolicyId,
    outcome: AuthorizationDecisionOutcome,
}

impl PolicyResultReference {
    pub fn new(policy_id: PolicyId, outcome: AuthorizationDecisionOutcome) -> Self {
        Self { policy_id, outcome }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritySourceReference {
    policy_result: PolicyResultReference,
    authorization_decision: AuthorizationDecisionReference,
}

impl AuthoritySourceReference {
    pub fn new(
        policy_result: PolicyResultReference,
        authorization_decision: AuthorizationDecisionReference,
    ) -> Self {
        Self {
            policy_result,
            authorization_decision,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDutiesConflict {
    rule: NonEmptyText,
}

impl SeparationOfDutiesConflict {
    pub fn new(rule: impl Into<String>) -> DomainResult<Self> {
        Ok(Self { rule: NonEmptyText::new("sod_rule", rule)? })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationReference {
    delegation_id: DelegationId,
    namespace: EnglishNamespace,
    version: DelegationVersion,
    delegator: DelegatorReference,
    delegate: DelegateReference,
    beneficiary: BeneficiaryReference,
    authority_source: AuthoritySourceReference,
    scope: DelegationScope,
    delegated_rights: Vec<DelegatedRightReference>,
    delegated_tasks: Vec<DelegatedTaskReference>,
    conditions: Vec<DelegationConditionReference>,
    depth: DelegationDepth,
    lifecycle: DelegationLifecycle,
    separation_of_duties: Option<SeparationOfDutiesConflict>,
}

impl DelegationReference {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        delegation_id: DelegationId,
        namespace: EnglishNamespace,
        version: DelegationVersion,
        delegator: DelegatorReference,
        delegate: DelegateReference,
        beneficiary: BeneficiaryReference,
        authority_source: AuthoritySourceReference,
        scope: DelegationScope,
        delegated_rights: Vec<DelegatedRightReference>,
        delegated_tasks: Vec<DelegatedTaskReference>,
        conditions: Vec<DelegationConditionReference>,
        depth: DelegationDepth,
        lifecycle: DelegationLifecycle,
        separation_of_duties: Option<SeparationOfDutiesConflict>,
    ) -> DomainResult<Self> {
        if delegator.agent_id() == delegate.agent_id() {
            return Err(DomainError::InvalidDelegationReference(
                "delegator and delegate must remain distinct references",
            ));
        }
        if depth.value() == 0 && matches!(lifecycle, DelegationLifecycle::Active) {
            return Err(DomainError::InvalidDelegationReference(
                "active delegation depth must represent at least one delegation hop",
            ));
        }
        if delegated_rights.is_empty() && delegated_tasks.is_empty() {
            return Err(DomainError::InvalidDelegationReference(
                "delegation must define delegated rights or tasks",
            ));
        }
        Ok(Self {
            delegation_id,
            namespace,
            version,
            delegator,
            delegate,
            beneficiary,
            authority_source,
            scope,
            delegated_rights,
            delegated_tasks,
            conditions,
            depth,
            lifecycle,
            separation_of_duties,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::{AgentCategory, AgentDefinition, AgentReference, AgentRuntimeReference, AgentType};
    use crate::authorization::{
        ActionVerb, AuthorizationDecisionOutcome, AuthorizationDecisionReference, PermissionEffectIntent,
        PermissionReference,
    };
    use crate::identifier::{
        AgentId, AgentUuid, AuthorizationDecisionId, DelegationId, EnglishNamespace, EnterpriseId,
        HumanId, PermissionId, PolicyId, StableVersion,
    };
    use crate::identity::AgentIdentity;
    use crate::lifecycle::{AgentLifecycle, DelegationLifecycle};
    use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};

    use super::{
        AuthoritySourceReference, BeneficiaryReference, DelegateReference, DelegatedRightReference,
        DelegationDepth, DelegationReference, DelegationScope, DelegationScopeKind, DelegationVersion,
        DelegatorReference, PolicyResultReference,
    };

    fn agent_reference(agent_suffix: &str) -> AgentReference {
        let enterprise = EnterpriseId::new("CX-ENT-000001").expect("enterprise");
        let path = OwnershipPath::new(
            enterprise.clone(),
            Some(crate::identifier::WorkspaceId::new("CX-WS-000001").expect("workspace")),
            None,
            None,
        )
        .expect("path");
        let owner = OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"));
        let context = OrganizationalContext::new(path, owner.clone());
        let identity = AgentIdentity::new(
            AgentId::new(format!("CX-AGT-{agent_suffix}")).expect("agent"),
            EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            enterprise,
            AgentLifecycle::Active,
        )
        .expect("identity");
        let definition = AgentDefinition::new(
            identity.clone(),
            AgentUuid::new("CX-UUID-00000001").expect("uuid"),
            "Kernel Agent",
            AgentType::new("Supervisor").expect("type"),
            AgentCategory::new("Operations").expect("category"),
            owner,
            context,
            AgentRuntimeReference::new("runtime-ref").expect("runtime"),
        )
        .expect("definition");
        let _ = definition;
        AgentReference::new(identity)
    }

    fn permission() -> PermissionReference {
        PermissionReference::new(
            PermissionId::new("CX-PERM-000001").expect("permission"),
            ActionVerb::new("approve").expect("verb"),
            crate::authorization::ResourceType::new("workflow").expect("type"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    #[test]
    fn delegation_creates_valid_reference_ces_b0_029_1() {
        let delegator = DelegatorReference::new(agent_reference("000001"));
        let delegate = DelegateReference::new(agent_reference("000002"));
        let delegation = DelegationReference::new(
            DelegationId::new("CX-DEL-000001").expect("id"),
            EnglishNamespace::new("delegation_namespace", "enterprise.delegation").expect("namespace"),
            DelegationVersion::new("1.0.0").expect("version"),
            delegator,
            delegate.clone(),
            BeneficiaryReference::Delegate(delegate),
            AuthoritySourceReference::new(
                PolicyResultReference::new(
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                ),
                AuthorizationDecisionReference::new(
                    AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                ),
            ),
            DelegationScope::new(
                DelegationScopeKind::Workspace,
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
            vec![DelegatedRightReference::new(permission(), "approve workflow").expect("right")],
            vec![],
            vec![],
            DelegationDepth::new(1).expect("depth"),
            DelegationLifecycle::Accepted,
            None,
        )
        .expect("delegation");
        let _ = delegation;
    }

    #[test]
    fn delegation_rejects_invalid_depth_ces_b0_029_9() {
        let error = DelegationDepth::new(-1).expect_err("negative depth must fail");
        assert!(error.to_string().contains("delegation depth cannot be negative"));
    }

    #[test]
    fn delegation_rejects_empty_scope_ces_b0_029_5() {
        let error = DelegationScope::new(
            DelegationScopeKind::Agent,
            OwnershipPath::new(
                EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
                None,
                None,
                None,
            )
            .expect("path"),
            None,
        )
        .expect_err("agent scope without id must fail");
        assert!(error.to_string().contains("agent scope requires an agent identifier"));
    }

    #[test]
    fn delegation_rejects_self_delegation_traceability_k1() {
        let agent = agent_reference("000001");
        let error = DelegationReference::new(
            DelegationId::new("CX-DEL-000001").expect("id"),
            EnglishNamespace::new("delegation_namespace", "enterprise.delegation").expect("namespace"),
            DelegationVersion::new("1.0.0").expect("version"),
            DelegatorReference::new(agent.clone()),
            DelegateReference::new(agent.clone()),
            BeneficiaryReference::Delegate(DelegateReference::new(agent)),
            AuthoritySourceReference::new(
                PolicyResultReference::new(
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                ),
                AuthorizationDecisionReference::new(
                    AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                ),
            ),
            DelegationScope::new(
                DelegationScopeKind::Workspace,
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
            vec![DelegatedRightReference::new(permission(), "approve workflow").expect("right")],
            vec![],
            vec![],
            DelegationDepth::new(1).expect("depth"),
            DelegationLifecycle::Accepted,
            None,
        )
        .expect_err("self-delegation must fail");
        assert!(error.to_string().contains("delegator and delegate must remain distinct references"));
    }
}
