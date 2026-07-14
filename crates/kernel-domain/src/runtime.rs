use crate::agent::{AgentDefinition, AgentRuntimeReference};
use crate::authorization::PermissionReference;
use crate::errors::{DomainError, DomainResult};
use crate::identifier::{AgentId, CapabilityId, HeartbeatId, LeaseId, PolicyId, RuntimeId};
use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
use crate::request::TimeReference;
use crate::{EnterpriseId, NonEmptyText};
use std::collections::{BTreeMap, BTreeSet};

/// CES Traceability: CES-B0-027.6, CES-B0-027.10, CES-B0-027.18
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeHealth {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.10, K4.1 runtime presence baseline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceState {
    Registered,
    Ready,
    Idle,
    Working,
    Paused,
    Recovering,
    Offline,
    Retired,
}

impl PresenceState {
    pub fn can_transition_to(self, target: Self) -> DomainResult<()> {
        let allowed = matches!(
            (self, target),
            (
                Self::Registered,
                Self::Ready | Self::Offline | Self::Retired
            ) | (
                Self::Ready,
                Self::Idle | Self::Paused | Self::Offline | Self::Retired
            ) | (
                Self::Idle,
                Self::Working | Self::Paused | Self::Offline | Self::Retired
            ) | (
                Self::Working,
                Self::Idle | Self::Paused | Self::Recovering | Self::Offline | Self::Retired
            ) | (
                Self::Paused,
                Self::Ready | Self::Idle | Self::Recovering | Self::Offline | Self::Retired
            ) | (
                Self::Recovering,
                Self::Ready | Self::Idle | Self::Offline | Self::Retired
            ) | (Self::Offline, Self::Recovering | Self::Retired)
        );
        if allowed {
            Ok(())
        } else {
            Err(DomainError::InvalidRuntimeReference(
                "presence transition is not allowed by the K4.1 runtime lifecycle map",
            ))
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }
}

/// CES Traceability: CES-B0-027.5
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptor {
    capability_id: CapabilityId,
    description: NonEmptyText,
    dependencies: Vec<NonEmptyText>,
    inputs: Vec<NonEmptyText>,
    outputs: Vec<NonEmptyText>,
    required_permissions: Vec<PermissionReference>,
    governing_policies: Vec<PolicyId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityDescriptorSpec {
    pub capability_id: CapabilityId,
    pub description: String,
    pub dependencies: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub required_permissions: Vec<PermissionReference>,
    pub governing_policies: Vec<PolicyId>,
}

impl CapabilityDescriptor {
    pub fn new(spec: CapabilityDescriptorSpec) -> DomainResult<Self> {
        if spec.dependencies.is_empty() || spec.inputs.is_empty() || spec.outputs.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "capability descriptors require dependencies, inputs, and outputs",
            ));
        }
        if spec.required_permissions.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "capability descriptors require at least one permission reference",
            ));
        }
        Ok(Self {
            capability_id: spec.capability_id,
            description: NonEmptyText::new("capability_description", spec.description)?,
            dependencies: normalize_text_list("capability_dependency", spec.dependencies)?,
            inputs: normalize_text_list("capability_input", spec.inputs)?,
            outputs: normalize_text_list("capability_output", spec.outputs)?,
            required_permissions: spec.required_permissions,
            governing_policies: spec.governing_policies,
        })
    }

    pub fn capability_id(&self) -> &CapabilityId {
        &self.capability_id
    }
}

/// CES Traceability: CES-B0-027.1, CES-B0-027.13, K4.1 runtime foundation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEntity {
    runtime_id: RuntimeId,
    runtime_reference: AgentRuntimeReference,
    enterprise_id: EnterpriseId,
    ownership_path: OwnershipPath,
    health: RuntimeHealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEntitySpec {
    pub runtime_id: RuntimeId,
    pub runtime_reference: AgentRuntimeReference,
    pub enterprise_id: EnterpriseId,
    pub ownership_path: OwnershipPath,
    pub health: RuntimeHealth,
}

impl RuntimeEntity {
    pub fn new(spec: RuntimeEntitySpec) -> DomainResult<Self> {
        if spec.enterprise_id != *spec.ownership_path.enterprise_id() {
            return Err(DomainError::InvalidRuntimeReference(
                "runtime entity enterprise must match its ownership path enterprise",
            ));
        }
        Ok(Self {
            runtime_id: spec.runtime_id,
            runtime_reference: spec.runtime_reference,
            enterprise_id: spec.enterprise_id,
            ownership_path: spec.ownership_path,
            health: spec.health,
        })
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn ownership_path(&self) -> &OwnershipPath {
        &self.ownership_path
    }

    pub fn health(&self) -> RuntimeHealth {
        self.health
    }
}

/// CES Traceability: CES-B0-027.7, CES-B0-027.8
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRecord {
    lease_id: LeaseId,
    runtime_id: RuntimeId,
    issued_at: TimeReference,
    expires_at: TimeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRecordSpec {
    pub lease_id: LeaseId,
    pub runtime_id: RuntimeId,
    pub issued_at: TimeReference,
    pub expires_at: TimeReference,
}

impl LeaseRecord {
    pub fn new(spec: LeaseRecordSpec) -> DomainResult<Self> {
        if spec.expires_at.as_str() <= spec.issued_at.as_str() {
            return Err(DomainError::InvalidRuntimeReference(
                "lease expiration must be after lease issuance",
            ));
        }
        Ok(Self {
            lease_id: spec.lease_id,
            runtime_id: spec.runtime_id,
            issued_at: spec.issued_at,
            expires_at: spec.expires_at,
        })
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn is_current_at(&self, reference_time: &TimeReference) -> bool {
        self.issued_at.as_str() <= reference_time.as_str()
            && reference_time.as_str() < self.expires_at.as_str()
    }
}

/// CES Traceability: CES-B0-027.6, CES-B0-027.10, CES-B0-027.17
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRecord {
    heartbeat_id: HeartbeatId,
    runtime_id: RuntimeId,
    recorded_at: TimeReference,
    fresh_until: TimeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeartbeatRecordSpec {
    pub heartbeat_id: HeartbeatId,
    pub runtime_id: RuntimeId,
    pub recorded_at: TimeReference,
    pub fresh_until: TimeReference,
}

impl HeartbeatRecord {
    pub fn new(spec: HeartbeatRecordSpec) -> DomainResult<Self> {
        if spec.fresh_until.as_str() < spec.recorded_at.as_str() {
            return Err(DomainError::InvalidRuntimeReference(
                "heartbeat freshness must not precede the recorded timestamp",
            ));
        }
        Ok(Self {
            heartbeat_id: spec.heartbeat_id,
            runtime_id: spec.runtime_id,
            recorded_at: spec.recorded_at,
            fresh_until: spec.fresh_until,
        })
    }

    pub fn runtime_id(&self) -> &RuntimeId {
        &self.runtime_id
    }

    pub fn recorded_at(&self) -> &TimeReference {
        &self.recorded_at
    }

    pub fn is_fresh_at(&self, reference_time: &TimeReference) -> bool {
        reference_time.as_str() <= self.fresh_until.as_str()
    }
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.15, CES-B0-027.22
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRegistration {
    agent: AgentDefinition,
    runtime: RuntimeEntity,
    supervisor: OwnerReference,
    capabilities: Vec<CapabilityDescriptor>,
    presence_state: PresenceState,
    health: RuntimeHealth,
    registered_at: TimeReference,
    lease: Option<LeaseRecord>,
    last_heartbeat: Option<HeartbeatRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRegistrationSpec {
    pub agent: AgentDefinition,
    pub runtime: RuntimeEntity,
    pub supervisor: OwnerReference,
    pub capabilities: Vec<CapabilityDescriptor>,
    pub presence_state: PresenceState,
    pub health: RuntimeHealth,
    pub registered_at: TimeReference,
    pub lease: Option<LeaseRecord>,
    pub last_heartbeat: Option<HeartbeatRecord>,
}

impl AgentRegistration {
    pub fn new(spec: AgentRegistrationSpec) -> DomainResult<Self> {
        if spec.capabilities.is_empty() {
            return Err(DomainError::InvalidRuntimeReference(
                "agent registration requires at least one capability",
            ));
        }
        if spec.presence_state != PresenceState::Registered {
            return Err(DomainError::InvalidRuntimeReference(
                "new agent registrations must begin in Registered state",
            ));
        }
        if spec.agent.identity().enterprise_id() != &spec.runtime.enterprise_id {
            return Err(DomainError::InvalidRuntimeReference(
                "agent identity enterprise must match runtime enterprise",
            ));
        }
        let agent_context = spec.agent.organizational_context();
        if !ownership_contains(
            spec.runtime.ownership_path(),
            agent_context.ownership_path(),
        ) {
            return Err(DomainError::InvalidRuntimeReference(
                "runtime ownership path must contain the agent governance path",
            ));
        }
        if let Some(lease) = &spec.lease {
            if lease.runtime_id() != spec.runtime.runtime_id() {
                return Err(DomainError::InvalidRuntimeReference(
                    "lease runtime must match the registration runtime",
                ));
            }
        }
        if let Some(heartbeat) = &spec.last_heartbeat {
            if heartbeat.runtime_id() != spec.runtime.runtime_id() {
                return Err(DomainError::InvalidRuntimeReference(
                    "heartbeat runtime must match the registration runtime",
                ));
            }
        }
        Ok(Self {
            agent: spec.agent,
            runtime: spec.runtime,
            supervisor: spec.supervisor,
            capabilities: spec.capabilities,
            presence_state: spec.presence_state,
            health: spec.health,
            registered_at: spec.registered_at,
            lease: spec.lease,
            last_heartbeat: spec.last_heartbeat,
        })
    }

    pub fn agent(&self) -> &AgentDefinition {
        &self.agent
    }

    pub fn agent_id(&self) -> &AgentId {
        self.agent.identity().agent_id()
    }

    pub fn runtime(&self) -> &RuntimeEntity {
        &self.runtime
    }

    pub fn organizational_context(&self) -> &OrganizationalContext {
        self.agent.organizational_context()
    }

    pub fn capabilities(&self) -> &[CapabilityDescriptor] {
        &self.capabilities
    }

    pub fn presence_state(&self) -> PresenceState {
        self.presence_state
    }

    pub fn health(&self) -> RuntimeHealth {
        self.health
    }

    pub fn lease(&self) -> Option<&LeaseRecord> {
        self.lease.as_ref()
    }

    pub fn last_heartbeat(&self) -> Option<&HeartbeatRecord> {
        self.last_heartbeat.as_ref()
    }

    pub fn transition_presence(&mut self, next_state: PresenceState) -> DomainResult<()> {
        self.presence_state.can_transition_to(next_state)?;
        self.presence_state = next_state;
        Ok(())
    }

    pub fn replace_lease(&mut self, lease: LeaseRecord) -> DomainResult<()> {
        if lease.runtime_id() != self.runtime.runtime_id() {
            return Err(DomainError::InvalidRuntimeReference(
                "lease runtime must match the registered runtime",
            ));
        }
        self.lease = Some(lease);
        Ok(())
    }

    pub fn record_heartbeat(&mut self, heartbeat: HeartbeatRecord) -> DomainResult<()> {
        if heartbeat.runtime_id() != self.runtime.runtime_id() {
            return Err(DomainError::InvalidRuntimeReference(
                "heartbeat runtime must match the registered runtime",
            ));
        }
        if self.presence_state.is_terminal() {
            return Err(DomainError::InvalidRuntimeRegistry(
                "retired registrations must not receive new heartbeat records",
            ));
        }
        self.last_heartbeat = Some(heartbeat);
        Ok(())
    }

    pub fn lease_is_current_at(&self, reference_time: &TimeReference) -> bool {
        self.lease
            .as_ref()
            .map(|lease| lease.is_current_at(reference_time))
            .unwrap_or(false)
    }
}

/// CES Traceability: CES-B0-027.8, CES-B0-027.9, CES-B0-027.21, CES-B0-027.22
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AgentRegistry {
    registrations_by_agent_id: BTreeMap<AgentId, AgentRegistration>,
    capability_index: BTreeMap<CapabilityId, BTreeSet<AgentId>>,
    runtime_index: BTreeMap<RuntimeId, BTreeSet<AgentId>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, registration: AgentRegistration) -> DomainResult<()> {
        let agent_id = registration.agent_id().clone();
        if self.registrations_by_agent_id.contains_key(&agent_id) {
            return Err(DomainError::InvalidRuntimeRegistry(
                "agent registration requires unique agent identity",
            ));
        }
        let runtime_id = registration.runtime().runtime_id().clone();
        for capability in registration.capabilities() {
            self.capability_index
                .entry(capability.capability_id().clone())
                .or_default()
                .insert(agent_id.clone());
        }
        self.runtime_index
            .entry(runtime_id)
            .or_default()
            .insert(agent_id.clone());
        self.registrations_by_agent_id
            .insert(agent_id, registration);
        Ok(())
    }

    pub fn lookup(&self, agent_id: &AgentId) -> Option<&AgentRegistration> {
        self.registrations_by_agent_id.get(agent_id)
    }

    pub fn registrations_for_capability(
        &self,
        capability_id: &CapabilityId,
    ) -> Vec<&AgentRegistration> {
        self.capability_index
            .get(capability_id)
            .into_iter()
            .flat_map(|agent_ids| agent_ids.iter())
            .filter_map(|agent_id| self.registrations_by_agent_id.get(agent_id))
            .collect()
    }

    pub fn registrations_for_runtime(&self, runtime_id: &RuntimeId) -> Vec<&AgentRegistration> {
        self.runtime_index
            .get(runtime_id)
            .into_iter()
            .flat_map(|agent_ids| agent_ids.iter())
            .filter_map(|agent_id| self.registrations_by_agent_id.get(agent_id))
            .collect()
    }

    pub fn transition_presence(
        &mut self,
        agent_id: &AgentId,
        next_state: PresenceState,
    ) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "presence transitions require an existing registration",
            ),
        )?;
        registration.transition_presence(next_state)
    }

    pub fn renew_lease(&mut self, agent_id: &AgentId, lease: LeaseRecord) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry("lease renewal requires an existing registration"),
        )?;
        registration.replace_lease(lease)
    }

    pub fn record_heartbeat(
        &mut self,
        agent_id: &AgentId,
        heartbeat: HeartbeatRecord,
    ) -> DomainResult<()> {
        let registration = self.registrations_by_agent_id.get_mut(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "heartbeat updates require an existing registration",
            ),
        )?;
        registration.record_heartbeat(heartbeat)
    }

    pub fn lease_is_current_at(
        &self,
        agent_id: &AgentId,
        reference_time: &TimeReference,
    ) -> DomainResult<bool> {
        let registration = self.registrations_by_agent_id.get(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry(
                "lease validation requires an existing registration",
            ),
        )?;
        Ok(registration.lease_is_current_at(reference_time))
    }

    pub fn deregister(&mut self, agent_id: &AgentId) -> DomainResult<AgentRegistration> {
        let mut registration = self.registrations_by_agent_id.remove(agent_id).ok_or(
            DomainError::InvalidRuntimeRegistry("deregistration requires an existing registration"),
        )?;
        let _ = registration.transition_presence(PresenceState::Retired);
        for capability in registration.capabilities() {
            if let Some(agent_ids) = self.capability_index.get_mut(capability.capability_id()) {
                agent_ids.remove(agent_id);
                if agent_ids.is_empty() {
                    self.capability_index.remove(capability.capability_id());
                }
            }
        }
        if let Some(agent_ids) = self
            .runtime_index
            .get_mut(registration.runtime().runtime_id())
        {
            agent_ids.remove(agent_id);
            if agent_ids.is_empty() {
                self.runtime_index
                    .remove(registration.runtime().runtime_id());
            }
        }
        Ok(registration)
    }
}

fn normalize_text_list(
    field: &'static str,
    values: Vec<String>,
) -> DomainResult<Vec<NonEmptyText>> {
    values
        .into_iter()
        .map(|value| NonEmptyText::new(field, value))
        .collect()
}

fn ownership_contains(parent: &OwnershipPath, child: &OwnershipPath) -> bool {
    if parent.enterprise_id() != child.enterprise_id() {
        return false;
    }
    if parent.workspace_id().is_some() && parent.workspace_id() != child.workspace_id() {
        return false;
    }
    if parent.project_id().is_some() && parent.project_id() != child.project_id() {
        return false;
    }
    if parent.organizational_unit_id().is_some()
        && parent.organizational_unit_id() != child.organizational_unit_id()
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{
        AgentRegistration, AgentRegistrationSpec, AgentRegistry, CapabilityDescriptor,
        CapabilityDescriptorSpec, HeartbeatRecord, HeartbeatRecordSpec, LeaseRecord,
        LeaseRecordSpec, PresenceState, RuntimeEntity, RuntimeEntitySpec, RuntimeHealth,
    };
    use crate::agent::{
        AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentRuntimeReference, AgentType,
    };
    use crate::authorization::{
        ActionVerb, PermissionEffectIntent, PermissionReference, ResourceType,
    };
    use crate::identifier::{
        AgentId, AgentUuid, CapabilityId, EnglishNamespace, EnterpriseId, HeartbeatId, HumanId,
        LeaseId, PermissionId, PolicyId, ProjectId, RuntimeId, StableVersion, WorkspaceId,
    };
    use crate::identity::AgentIdentity;
    use crate::lifecycle::AgentLifecycle;
    use crate::ownership::{OrganizationalContext, OwnerReference, OwnershipPath};
    use crate::request::TimeReference;

    fn enterprise_id() -> EnterpriseId {
        EnterpriseId::new("CX-ENT-000001").expect("enterprise")
    }

    fn owner() -> OwnerReference {
        OwnerReference::new(HumanId::new("CX-EMP-000001").expect("owner"))
    }

    fn ownership_path(project_id: &str) -> OwnershipPath {
        OwnershipPath::new(
            enterprise_id(),
            Some(WorkspaceId::new("CX-WS-000001").expect("workspace")),
            Some(ProjectId::new(project_id).expect("project")),
            None,
        )
        .expect("path")
    }

    fn permission(permission_id: &str, action: &str) -> PermissionReference {
        PermissionReference::new(
            PermissionId::new(permission_id).expect("permission"),
            ActionVerb::new(action).expect("action"),
            ResourceType::new("workflow").expect("resource"),
            PermissionEffectIntent::new("Permit").expect("effect"),
        )
    }

    fn capability(capability_id: &str) -> CapabilityDescriptor {
        CapabilityDescriptor::new(CapabilityDescriptorSpec {
            capability_id: CapabilityId::new(capability_id).expect("capability"),
            description: "render governed output".to_owned(),
            dependencies: vec!["model".to_owned()],
            inputs: vec!["prompt".to_owned()],
            outputs: vec!["artifact".to_owned()],
            required_permissions: vec![permission("CX-PERM-000001", "approve")],
            governing_policies: vec![PolicyId::new("CX-POL-000001").expect("policy")],
        })
        .expect("capability")
    }

    fn runtime_entity(runtime_id: &str, project_id: &str) -> RuntimeEntity {
        RuntimeEntity::new(RuntimeEntitySpec {
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            runtime_reference: AgentRuntimeReference::new("runtime.primary").expect("runtime ref"),
            enterprise_id: enterprise_id(),
            ownership_path: ownership_path(project_id),
            health: RuntimeHealth::Healthy,
        })
        .expect("runtime entity")
    }

    fn agent_definition(agent_id: &str, project_id: &str) -> AgentDefinition {
        AgentDefinition::new(AgentDefinitionSpec {
            identity: AgentIdentity::new(
                AgentId::new(agent_id).expect("agent"),
                EnglishNamespace::new("agent_namespace", "enterprise.agent").expect("namespace"),
                StableVersion::new("agent_version", "1.0.0").expect("version"),
                enterprise_id(),
                AgentLifecycle::Registered,
            )
            .expect("identity"),
            agent_uuid: AgentUuid::new("CX-UUID-00000001").expect("uuid"),
            agent_name: "Agent One".to_owned(),
            agent_type: AgentType::new("worker").expect("type"),
            agent_category: AgentCategory::new("creative").expect("category"),
            owner: owner(),
            organizational_context: OrganizationalContext::new(ownership_path(project_id), owner()),
            runtime_reference: AgentRuntimeReference::new("runtime.primary").expect("runtime ref"),
        })
        .expect("agent definition")
    }

    fn lease(runtime_id: &str, expires_at: &str) -> LeaseRecord {
        LeaseRecord::new(LeaseRecordSpec {
            lease_id: LeaseId::new("CX-LEASE-000001").expect("lease"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            issued_at: TimeReference::new("2026-07-15T00:00:00Z").expect("issued"),
            expires_at: TimeReference::new(expires_at).expect("expires"),
        })
        .expect("lease")
    }

    fn heartbeat(runtime_id: &str, fresh_until: &str) -> HeartbeatRecord {
        HeartbeatRecord::new(HeartbeatRecordSpec {
            heartbeat_id: HeartbeatId::new("CX-HB-000001").expect("heartbeat"),
            runtime_id: RuntimeId::new(runtime_id).expect("runtime"),
            recorded_at: TimeReference::new("2026-07-15T00:10:00Z").expect("recorded"),
            fresh_until: TimeReference::new(fresh_until).expect("fresh until"),
        })
        .expect("heartbeat")
    }

    fn registration(agent_id: &str, capability_id: &str, runtime_id: &str) -> AgentRegistration {
        AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition(agent_id, "CX-PROJ-000001"),
            runtime: runtime_entity(runtime_id, "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![capability(capability_id)],
            presence_state: PresenceState::Registered,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease: Some(lease(runtime_id, "2026-07-15T01:00:00Z")),
            last_heartbeat: None,
        })
        .expect("registration")
    }

    #[test]
    fn runtime_registration_is_stable_and_lookup_is_deterministic_ces_b0_027_8() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let lookup = registry.lookup(&agent_id).expect("lookup");
        assert_eq!(lookup.agent_id().as_str(), "CX-AGT-000001");
    }

    #[test]
    fn runtime_registration_rejects_duplicate_agent_identity_ces_b0_027_8() {
        let mut registry = AgentRegistry::new();
        registry
            .register(registration(
                "CX-AGT-000001",
                "CX-CAP-000001",
                "runtime.primary",
            ))
            .expect("first registration");
        let error = registry
            .register(registration(
                "CX-AGT-000001",
                "CX-CAP-000002",
                "runtime.primary",
            ))
            .expect_err("duplicate registration must fail");
        assert!(error
            .to_string()
            .contains("agent registration requires unique agent identity"));
    }

    #[test]
    fn runtime_capability_lookup_uses_indexed_registration_state_ces_b0_027_9() {
        let mut registry = AgentRegistry::new();
        registry
            .register(registration(
                "CX-AGT-000001",
                "CX-CAP-000001",
                "runtime.primary",
            ))
            .expect("register one");
        registry
            .register(registration(
                "CX-AGT-000002",
                "CX-CAP-000001",
                "runtime.primary",
            ))
            .expect("register two");
        let registrations = registry
            .registrations_for_capability(&CapabilityId::new("CX-CAP-000001").expect("cap"))
            .iter()
            .map(|registration| registration.agent_id().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            registrations,
            vec!["CX-AGT-000001".to_owned(), "CX-AGT-000002".to_owned()]
        );
    }

    #[test]
    fn runtime_runtime_lookup_groups_registrations_by_runtime_id_k4_1() {
        let mut registry = AgentRegistry::new();
        registry
            .register(registration(
                "CX-AGT-000001",
                "CX-CAP-000001",
                "runtime.primary",
            ))
            .expect("register one");
        registry
            .register(registration(
                "CX-AGT-000002",
                "CX-CAP-000002",
                "runtime.secondary",
            ))
            .expect("register two");
        assert_eq!(
            registry
                .registrations_for_runtime(&RuntimeId::new("runtime.primary").expect("runtime"))
                .len(),
            1
        );
    }

    #[test]
    fn runtime_presence_follows_k4_1_transition_order() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        registry
            .transition_presence(&agent_id, PresenceState::Ready)
            .expect("ready");
        registry
            .transition_presence(&agent_id, PresenceState::Idle)
            .expect("idle");
        registry
            .transition_presence(&agent_id, PresenceState::Working)
            .expect("working");
        assert_eq!(
            registry.lookup(&agent_id).expect("lookup").presence_state(),
            PresenceState::Working
        );
    }

    #[test]
    fn runtime_presence_rejects_invalid_transition_k4_1() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let error = registry
            .transition_presence(&agent_id, PresenceState::Working)
            .expect_err("registered must not jump directly to working");
        assert!(error
            .to_string()
            .contains("presence transition is not allowed"));
    }

    #[test]
    fn runtime_lease_validation_detects_current_and_expired_leases_ces_b0_027_7() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        assert!(registry
            .lease_is_current_at(
                &agent_id,
                &TimeReference::new("2026-07-15T00:30:00Z").expect("time")
            )
            .expect("current lease"));
        assert!(!registry
            .lease_is_current_at(
                &agent_id,
                &TimeReference::new("2026-07-15T02:00:00Z").expect("time")
            )
            .expect("expired lease"));
    }

    #[test]
    fn runtime_lease_renewal_rejects_runtime_mismatch_k4_1() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let error = registry
            .renew_lease(
                &agent_id,
                lease("runtime.secondary", "2026-07-15T03:00:00Z"),
            )
            .expect_err("renewal must respect runtime binding");
        assert!(error
            .to_string()
            .contains("lease runtime must match the registered runtime"));
    }

    #[test]
    fn runtime_heartbeat_updates_last_seen_state_ces_b0_027_10() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        registry
            .record_heartbeat(
                &agent_id,
                heartbeat("runtime.primary", "2026-07-15T00:20:00Z"),
            )
            .expect("heartbeat");
        assert_eq!(
            registry
                .lookup(&agent_id)
                .expect("lookup")
                .last_heartbeat()
                .expect("heartbeat")
                .recorded_at()
                .as_str(),
            "2026-07-15T00:10:00Z"
        );
    }

    #[test]
    fn runtime_heartbeat_rejects_runtime_mismatch_k4_1() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let error = registry
            .record_heartbeat(
                &agent_id,
                heartbeat("runtime.secondary", "2026-07-15T00:20:00Z"),
            )
            .expect_err("heartbeat runtime must match");
        assert!(error
            .to_string()
            .contains("heartbeat runtime must match the registered runtime"));
    }

    #[test]
    fn runtime_deregistration_removes_capability_indexes_ces_b0_027_9() {
        let mut registry = AgentRegistry::new();
        let registration = registration("CX-AGT-000001", "CX-CAP-000001", "runtime.primary");
        let agent_id = registration.agent_id().clone();
        registry.register(registration).expect("register");
        let retired = registry.deregister(&agent_id).expect("deregister");
        assert_eq!(retired.presence_state(), PresenceState::Retired);
        assert!(registry.lookup(&agent_id).is_none());
        assert!(registry
            .registrations_for_capability(&CapabilityId::new("CX-CAP-000001").expect("cap"))
            .is_empty());
    }

    #[test]
    fn runtime_registration_requires_registered_initial_presence_k4_1() {
        let error = AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition("CX-AGT-000001", "CX-PROJ-000001"),
            runtime: runtime_entity("runtime.primary", "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![capability("CX-CAP-000001")],
            presence_state: PresenceState::Ready,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease: Some(lease("runtime.primary", "2026-07-15T01:00:00Z")),
            last_heartbeat: None,
        })
        .expect_err("registrations must begin in registered state");
        assert!(error
            .to_string()
            .contains("new agent registrations must begin in Registered state"));
    }

    #[test]
    fn runtime_registration_requires_capability_descriptors_ces_b0_027_5() {
        let error = AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition("CX-AGT-000001", "CX-PROJ-000001"),
            runtime: runtime_entity("runtime.primary", "CX-PROJ-000001"),
            supervisor: owner(),
            capabilities: vec![],
            presence_state: PresenceState::Registered,
            health: RuntimeHealth::Healthy,
            registered_at: TimeReference::new("2026-07-15T00:00:00Z").expect("registered"),
            lease: Some(lease("runtime.primary", "2026-07-15T01:00:00Z")),
            last_heartbeat: None,
        })
        .expect_err("registrations require capabilities");
        assert!(error
            .to_string()
            .contains("agent registration requires at least one capability"));
    }

    #[test]
    fn runtime_heartbeat_freshness_is_deterministic_ces_b0_027_10() {
        let heartbeat = heartbeat("runtime.primary", "2026-07-15T00:20:00Z");
        assert!(heartbeat.is_fresh_at(&TimeReference::new("2026-07-15T00:19:59Z").expect("time")));
        assert!(!heartbeat.is_fresh_at(&TimeReference::new("2026-07-15T00:20:01Z").expect("time")));
    }
}
