use crate::errors::DomainResult;
use crate::identifier::{AgentId, AgentUuid, NonEmptyText};
use crate::identity::AgentIdentity;
use crate::ownership::{OrganizationalContext, OwnerReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentType(NonEmptyText);

impl AgentType {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_type", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCategory(NonEmptyText);

impl AgentCategory {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_category", value)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRuntimeReference(NonEmptyText);

impl AgentRuntimeReference {
    pub fn new(value: impl Into<String>) -> DomainResult<Self> {
        Ok(Self(NonEmptyText::new("agent_runtime_reference", value)?))
    }
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
    pub fn new(
        identity: AgentIdentity,
        agent_uuid: AgentUuid,
        agent_name: impl Into<String>,
        agent_type: AgentType,
        agent_category: AgentCategory,
        owner: OwnerReference,
        organizational_context: OrganizationalContext,
        runtime_reference: AgentRuntimeReference,
    ) -> DomainResult<Self> {
        Ok(Self {
            identity,
            agent_uuid,
            agent_name: NonEmptyText::new("agent_name", agent_name)?,
            agent_type,
            agent_category,
            owner,
            organizational_context,
            runtime_reference,
        })
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
}
