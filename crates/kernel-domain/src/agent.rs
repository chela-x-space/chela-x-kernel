use crate::errors::{DomainError, DomainResult};
use crate::identifier::{AgentId, AgentUuid, NonEmptyText};
use crate::identity::AgentIdentity;
use crate::ownership::{OrganizationalContext, OwnerReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentType(NonEmptyText);

impl AgentType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_type", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCategory(NonEmptyText);

impl AgentCategory {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_category", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRuntimeReference(NonEmptyText);

impl AgentRuntimeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_runtime_reference", value)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinitionSpec {
    pub identity: AgentIdentity,
    pub agent_uuid: AgentUuid,
    pub agent_name: String,
    pub agent_type: AgentType,
    pub agent_category: AgentCategory,
    pub owner: OwnerReference,
    pub organizational_context: OrganizationalContext,
    pub runtime_reference: AgentRuntimeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDefinition {
    identity: AgentIdentity,
    agent_uuid: AgentUuid,
    agent_name: NonEmptyText,
    agent_type: AgentType,
    agent_category: AgentCategory,
    owner: OwnerReference,
    organizational_context: OrganizationalContext,
    runtime_reference: AgentRuntimeReference,
}

impl AgentDefinition {
    pub fn new(spec: AgentDefinitionSpec) -> DomainResult<Self> {
        Ok(Self {
            identity: spec.identity,
            agent_uuid: spec.agent_uuid,
            agent_name: NonEmptyText::new("agent_name", spec.agent_name)?,
            agent_type: spec.agent_type,
            agent_category: spec.agent_category,
            owner: spec.owner,
            organizational_context: spec.organizational_context,
            runtime_reference: spec.runtime_reference,
        })
    }

    pub fn identity(&self) -> &AgentIdentity {
        &self.identity
    }

    pub fn agent_uuid(&self) -> &AgentUuid {
        &self.agent_uuid
    }

    pub fn agent_name(&self) -> &str {
        self.agent_name.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentReference {
    identity: AgentIdentity,
}

impl AgentReference {
    pub fn new(identity: AgentIdentity) -> Self {
        Self { identity }
    }

    pub fn agent_id(&self) -> &AgentId {
        self.identity.agent_id()
    }

    pub fn identity(&self) -> &AgentIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFailureCategory {
    IdentityFailure,
    AuthorizationFailure,
    HeartbeatFailure,
    IsolationFailure,
    CommandFailure,
    DependencyFailure,
    AuditFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFailureSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentFailureReference {
    agent_id: AgentId,
    category: AgentFailureCategory,
    severity: AgentFailureSeverity,
    recovery_eligible: bool,
}

impl AgentFailureReference {
    pub fn new(
        agent_id: AgentId,
        category: AgentFailureCategory,
        severity: AgentFailureSeverity,
        recovery_eligible: bool,
    ) -> DomainResult<Self> {
        if matches!(severity, AgentFailureSeverity::Critical) && recovery_eligible {
            return Err(DomainError::InvalidAgentReference(
                "critical failures require supervisory handling before recovery eligibility is true",
            ));
        }
        Ok(Self {
            agent_id,
            category,
            severity,
            recovery_eligible,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRecoveryPlanReference(NonEmptyText);

impl AgentRecoveryPlanReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_recovery_plan", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRecoveryEvidenceReference(NonEmptyText);

impl AgentRecoveryEvidenceReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_recovery_evidence", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRecoveryReference {
    agent_id: AgentId,
    recovery_plan: AgentRecoveryPlanReference,
    supervising_owner: OwnerReference,
    evidence: AgentRecoveryEvidenceReference,
}

impl AgentRecoveryReference {
    pub fn new(
        agent_id: AgentId,
        recovery_plan: AgentRecoveryPlanReference,
        supervising_owner: OwnerReference,
        evidence: AgentRecoveryEvidenceReference,
    ) -> Self {
        Self {
            agent_id,
            recovery_plan,
            supervising_owner,
            evidence,
        }
    }
}
