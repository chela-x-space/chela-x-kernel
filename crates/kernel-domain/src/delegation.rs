use crate::agent::AgentReference;
use crate::authorization::{AuthorizationDecisionReference, PermissionReference};
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{DelegationId, EnglishNamespace, NonEmptyText, PolicyId, StableVersion};
use crate::lifecycle::DelegationLifecycle;
use crate::ownership::OwnershipPath;
use crate::policy::PolicyEffect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationVersion(StableVersion);

impl DelegationVersion {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(StableVersion::new("delegation_version", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
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
        if !matches!(kind, DelegationScopeKind::Agent) && agent_scope.is_some() {
            return Err(DomainError::InvalidDelegationReference(
                "non-agent delegation scope must not carry an agent identifier",
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

    pub fn kind(&self) -> &DelegationScopeKind {
        &self.kind
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
    pub fn new(
        permission: PermissionReference,
        responsibility: impl Into<String>,
    ) -> DomainResult<Self> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelegationDepth(u16);

impl DelegationDepth {
    pub fn new(value: i64) -> DomainResult<Self> {
        if value <= 0 {
            return Err(DomainError::InvalidDelegationReference(
                "delegation depth must be greater than zero",
            ));
        }
        Ok(Self(value as u16))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResultReference {
    policy_id: PolicyId,
    effect: PolicyEffect,
    explicit_deny: bool,
    non_waivable: bool,
}

impl PolicyResultReference {
    pub fn new(
        policy_id: PolicyId,
        effect: PolicyEffect,
        explicit_deny: bool,
        non_waivable: bool,
    ) -> DomainResult<Self> {
        if explicit_deny && !matches!(effect, PolicyEffect::Deny) {
            return Err(DomainError::InvalidPolicyReference(
                "explicit deny can only be represented with a deny effect",
            ));
        }
        Ok(Self {
            policy_id,
            effect,
            explicit_deny,
            non_waivable,
        })
    }

    pub fn effect(&self) -> PolicyEffect {
        self.effect
    }

    pub fn explicit_deny(&self) -> bool {
        self.explicit_deny
    }

    pub fn non_waivable(&self) -> bool {
        self.non_waivable
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
    ) -> DomainResult<Self> {
        if policy_result.explicit_deny() || !policy_result.effect().permits() {
            return Err(DomainError::InvalidDelegationReference(
                "delegation authority source requires a permitting policy result",
            ));
        }
        if authorization_decision.outcome().is_denied() {
            return Err(DomainError::InvalidDelegationReference(
                "delegation authority source requires an allowing authorization decision",
            ));
        }
        Ok(Self {
            policy_result,
            authorization_decision,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDutiesConflict {
    rule: NonEmptyText,
}

impl SeparationOfDutiesConflict {
    pub fn new(rule: impl Into<String>) -> DomainResult<Self> {
        Ok(Self {
            rule: NonEmptyText::new("sod_rule", rule)?,
        })
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationReferenceSpec {
    pub delegation_id: DelegationId,
    pub namespace: EnglishNamespace,
    pub version: DelegationVersion,
    pub delegator: DelegatorReference,
    pub delegate: DelegateReference,
    pub beneficiary: BeneficiaryReference,
    pub authority_source: AuthoritySourceReference,
    pub scope: DelegationScope,
    pub delegated_rights: Vec<DelegatedRightReference>,
    pub delegated_tasks: Vec<DelegatedTaskReference>,
    pub conditions: Vec<DelegationConditionReference>,
    pub depth: DelegationDepth,
    pub lifecycle: DelegationLifecycle,
    pub separation_of_duties: Option<SeparationOfDutiesConflict>,
}

impl DelegationReference {
    pub fn new(spec: DelegationReferenceSpec) -> DomainResult<Self> {
        if spec.depth.value() > 1 && spec.conditions.is_empty() {
            return Err(DomainError::InvalidDelegationReference(
                "re-delegation depth greater than one requires explicit policy authorization evidence",
            ));
        }
        if spec.delegated_rights.is_empty() && spec.delegated_tasks.is_empty() {
            return Err(DomainError::InvalidDelegationReference(
                "delegation must define delegated rights or tasks",
            ));
        }
        Ok(Self {
            delegation_id: spec.delegation_id,
            namespace: spec.namespace,
            version: spec.version,
            delegator: spec.delegator,
            delegate: spec.delegate,
            beneficiary: spec.beneficiary,
            authority_source: spec.authority_source,
            scope: spec.scope,
            delegated_rights: spec.delegated_rights,
            delegated_tasks: spec.delegated_tasks,
            conditions: spec.conditions,
            depth: spec.depth,
            lifecycle: spec.lifecycle,
            separation_of_duties: spec.separation_of_duties,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::agent::{
        AgentCategory, AgentDefinition, AgentReference, AgentRuntimeReference, AgentType,
    };
    use crate::authorization::{
        ActionVerb, AuthorizationDecisionOutcome, AuthorizationDecisionReference,
        PermissionEffectIntent, PermissionReference,
    };
    use crate::identifier::{
        AgentId, AgentUuid, AuthorizationDecisionId, DelegationId, EnglishNamespace, EnterpriseId,
        HumanId, PermissionId, PolicyId, StableVersion,
    };
    use crate::identity::AgentIdentity;
    use crate::lifecycle::{AgentLifecycle, DelegationLifecycle};
    use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
    use crate::policy::PolicyEffect;

    use super::{
        AuthoritySourceReference, BeneficiaryReference, DelegateReference, DelegatedRightReference,
        DelegationDepth, DelegationReference, DelegationReferenceSpec, DelegationScope,
        DelegationScopeKind, DelegationVersion, DelegatorReference, PolicyResultReference,
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
        let definition = AgentDefinition::new(crate::agent::AgentDefinitionSpec {
            identity: identity.clone(),
            agent_uuid: AgentUuid::new("CX-UUID-00000001").expect("uuid"),
            agent_name: "Kernel Agent".to_owned(),
            agent_type: AgentType::new("Supervisor").expect("type"),
            agent_category: AgentCategory::new("Operations").expect("category"),
            owner,
            organizational_context: context,
            runtime_reference: AgentRuntimeReference::new("runtime-ref").expect("runtime"),
        })
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
        let delegation = DelegationReference::new(DelegationReferenceSpec {
            delegation_id: DelegationId::new("CX-DEL-000001").expect("id"),
            namespace: EnglishNamespace::new("delegation_namespace", "enterprise.delegation")
                .expect("namespace"),
            version: DelegationVersion::new("1.0.0").expect("version"),
            delegator,
            delegate: delegate.clone(),
            beneficiary: BeneficiaryReference::Delegate(delegate),
            authority_source: AuthoritySourceReference::new(
                PolicyResultReference::new(
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    PolicyEffect::Permit,
                    false,
                    false,
                )
                .expect("policy result"),
                AuthorizationDecisionReference::new(
                    AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
                    crate::identifier::AuthorizationRequestId::new("CX-AUTHREQ-000001")
                        .expect("request"),
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                    crate::authorization::AuthorizationEvaluationOrderVersion::new("1.0.0")
                        .expect("order version"),
                    crate::authorization::MatchedPolicyEvidenceReference::new("policy-set-v1")
                        .expect("policy evidence"),
                    "2026-07-14T00:00:00Z",
                )
                .expect("authorization decision"),
            )
            .expect("authority source"),
            scope: DelegationScope::new(
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
            delegated_rights: vec![
                DelegatedRightReference::new(permission(), "approve workflow").expect("right"),
            ],
            delegated_tasks: vec![],
            conditions: vec![],
            depth: DelegationDepth::new(1).expect("depth"),
            lifecycle: DelegationLifecycle::Accepted,
            separation_of_duties: None,
        })
        .expect("delegation");
        let _ = delegation;
    }

    #[test]
    fn delegation_rejects_invalid_depth_ces_b0_029_9() {
        let error = DelegationDepth::new(0).expect_err("zero depth must fail");
        assert!(error
            .to_string()
            .contains("delegation depth must be greater than zero"));
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
        assert!(error
            .to_string()
            .contains("agent scope requires an agent identifier"));
    }

    #[test]
    fn delegation_requires_explicit_policy_evidence_for_redelegation_ces_b0_029_9() {
        let agent = agent_reference("000001");
        let error = DelegationReference::new(DelegationReferenceSpec {
            delegation_id: DelegationId::new("CX-DEL-000001").expect("id"),
            namespace: EnglishNamespace::new("delegation_namespace", "enterprise.delegation")
                .expect("namespace"),
            version: DelegationVersion::new("1.0.0").expect("version"),
            delegator: DelegatorReference::new(agent.clone()),
            delegate: DelegateReference::new(agent.clone()),
            beneficiary: BeneficiaryReference::Delegate(DelegateReference::new(agent)),
            authority_source: AuthoritySourceReference::new(
                PolicyResultReference::new(
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    PolicyEffect::Permit,
                    false,
                    false,
                )
                .expect("policy result"),
                AuthorizationDecisionReference::new(
                    AuthorizationDecisionId::new("CX-AUTHDEC-000001").expect("decision"),
                    crate::identifier::AuthorizationRequestId::new("CX-AUTHREQ-000001")
                        .expect("request"),
                    PolicyId::new("CX-POL-000001").expect("policy"),
                    AuthorizationDecisionOutcome::Allow,
                    crate::authorization::AuthorizationEvaluationOrderVersion::new("1.0.0")
                        .expect("order version"),
                    crate::authorization::MatchedPolicyEvidenceReference::new("policy-set-v1")
                        .expect("policy evidence"),
                    "2026-07-14T00:00:00Z",
                )
                .expect("authorization decision"),
            )
            .expect("authority source"),
            scope: DelegationScope::new(
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
            delegated_rights: vec![
                DelegatedRightReference::new(permission(), "approve workflow").expect("right"),
            ],
            delegated_tasks: vec![],
            conditions: vec![],
            depth: DelegationDepth::new(2).expect("depth"),
            lifecycle: DelegationLifecycle::Accepted,
            separation_of_duties: None,
        })
        .expect_err("re-delegation without explicit authorization evidence must fail");
        assert!(error
            .to_string()
            .contains("re-delegation depth greater than one"));
    }
}
