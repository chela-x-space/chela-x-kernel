use crate::errors::{DomainError, DomainResult};
use crate::identifier::{AgentId, EnglishNamespace, EnterpriseId, HumanId, StableVersion};
use crate::lifecycle::{AgentLifecycle, HumanLifecycle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityKind {
    Human,
    Agent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanIdentity {
    human_id: HumanId,
    enterprise_id: EnterpriseId,
    lifecycle: HumanLifecycle,
}

impl HumanIdentity {
    pub fn new(human_id: HumanId, enterprise_id: EnterpriseId, lifecycle: HumanLifecycle) -> Self {
        Self {
            human_id,
            enterprise_id,
            lifecycle,
        }
    }

    pub fn human_id(&self) -> &HumanId {
        &self.human_id
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn lifecycle(&self) -> HumanLifecycle {
        self.lifecycle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIdentity {
    agent_id: AgentId,
    namespace: EnglishNamespace,
    version: StableVersion,
    enterprise_id: EnterpriseId,
    lifecycle: AgentLifecycle,
}

impl AgentIdentity {
    pub fn new(
        agent_id: AgentId,
        namespace: EnglishNamespace,
        version: StableVersion,
        enterprise_id: EnterpriseId,
        lifecycle: AgentLifecycle,
    ) -> DomainResult<Self> {
        if matches!(lifecycle, AgentLifecycle::Deleted) {
            return Err(DomainError::InvalidIdentity(
                "new agent identity cannot begin in Deleted state",
            ));
        }
        Ok(Self {
            agent_id,
            namespace,
            version,
            enterprise_id,
            lifecycle,
        })
    }

    pub fn agent_id(&self) -> &AgentId {
        &self.agent_id
    }

    pub fn namespace(&self) -> &EnglishNamespace {
        &self.namespace
    }

    pub fn version(&self) -> &StableVersion {
        &self.version
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        &self.enterprise_id
    }

    pub fn lifecycle(&self) -> AgentLifecycle {
        self.lifecycle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityReference {
    Human(HumanIdentity),
    Agent(AgentIdentity),
}

impl IdentityReference {
    pub fn kind(&self) -> IdentityKind {
        match self {
            Self::Human(_) => IdentityKind::Human,
            Self::Agent(_) => IdentityKind::Agent,
        }
    }

    pub fn enterprise_id(&self) -> &EnterpriseId {
        match self {
            Self::Human(identity) => identity.enterprise_id(),
            Self::Agent(identity) => identity.enterprise_id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentIdentity, IdentityReference};
    use crate::identifier::{AgentId, EnglishNamespace, EnterpriseId, HumanId, StableVersion};
    use crate::lifecycle::{AgentLifecycle, HumanLifecycle};

    #[test]
    fn identity_creates_valid_human_identity_ces_b0_011_2() {
        let identity = super::HumanIdentity::new(
            HumanId::new("CX-EMP-000001").expect("human"),
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            HumanLifecycle::Registration,
        );
        assert_eq!(identity.human_id().as_str(), "CX-EMP-000001");
    }

    #[test]
    fn identity_rejects_invalid_agent_identity_ces_b0_027_2() {
        let error = AgentIdentity::new(
            AgentId::new("CX-AGT-000001").expect("agent"),
            EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            AgentLifecycle::Deleted,
        )
        .expect_err("deleted identity must fail");
        assert_eq!(
            error.to_string(),
            "invalid identity: new agent identity cannot begin in Deleted state"
        );
    }

    #[test]
    fn identity_id_is_immutable_through_public_api_ces_b0_027_2() {
        let identity = AgentIdentity::new(
            AgentId::new("CX-AGT-000001").expect("agent"),
            EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
            StableVersion::new("agent_version", "1.0.0").expect("version"),
            EnterpriseId::new("CX-ENT-000001").expect("enterprise"),
            AgentLifecycle::Registered,
        )
        .expect("valid identity");
        let reference = IdentityReference::Agent(identity.clone());
        assert_eq!(identity.agent_id().as_str(), "CX-AGT-000001");
        assert_eq!(reference.enterprise_id().as_str(), "CX-ENT-000001");
    }
}
