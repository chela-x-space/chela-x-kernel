use std::fmt::Debug;

use kernel_domain::{
    ActionVerb, AgentCategory, AgentDefinition, AgentDefinitionSpec, AgentId, AgentIdentity,
    AgentLifecycle, AgentRegistration, AgentRegistrationSpec, AgentRegistry, AgentRuntimeReference,
    AgentType, AgentUuid, CapabilityDescriptor, CapabilityDescriptorSpec, CapabilityId,
    EnglishNamespace, EnterpriseId, ExecutionSessionId, HeartbeatFreshnessPolicy, HeartbeatId,
    HeartbeatObservationSpec, HeartbeatRecord, HumanId, LeaseId, LeaseIssuanceSpec, LeaseRecord,
    OrganizationalContext, OwnerReference, OwnershipPath, PermissionEffectIntent, PermissionId,
    PermissionReference, PolicyId, PresenceState, ProjectId, ResourceType, RuntimeEntity,
    RuntimeEntitySpec, RuntimeHealth, RuntimeId, RuntimeStateSnapshot, StableVersion,
    TimeReference, WorkspaceId,
};
use kernel_studio::StudioRuntimeProjection;

use crate::projection_factory::host_local_studio_audit_reference;

fn checked<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

fn enterprise_id() -> Result<EnterpriseId, String> {
    checked(EnterpriseId::new("CX-ENT-000001"), "invalid enterprise id")
}

fn ownership_path() -> Result<OwnershipPath, String> {
    checked(
        OwnershipPath::new(
            enterprise_id()?,
            Some(checked(
                WorkspaceId::new("CX-WS-000001"),
                "invalid workspace id",
            )?),
            Some(checked(
                ProjectId::new("CX-PROJ-000001"),
                "invalid project id",
            )?),
            None,
        ),
        "invalid ownership path",
    )
}

fn time_reference(value: &str) -> Result<TimeReference, String> {
    checked(TimeReference::new(value), "invalid time reference")
}

fn permission_reference() -> Result<PermissionReference, String> {
    Ok(PermissionReference::new(
        checked(PermissionId::new("CX-PERM-000001"), "invalid permission id")?,
        checked(ActionVerb::new("read"), "invalid action verb")?,
        checked(ResourceType::new("runtime"), "invalid resource type")?,
        checked(
            PermissionEffectIntent::new("Permit"),
            "invalid permission effect",
        )?,
    ))
}

fn runtime_state_snapshot() -> Result<RuntimeStateSnapshot, String> {
    let agent_id = checked(AgentId::new("CX-AGT-000001"), "invalid agent id")?;
    let runtime_id = checked(RuntimeId::new("CX-RUN-000001"), "invalid runtime id")?;

    let identity = checked(
        AgentIdentity::new(
            agent_id.clone(),
            checked(
                EnglishNamespace::new("agent_namespace", "enterprise.agent"),
                "invalid agent namespace",
            )?,
            checked(
                StableVersion::new("agent_version", "1.0.0"),
                "invalid agent version",
            )?,
            enterprise_id()?,
            AgentLifecycle::Registered,
        ),
        "invalid agent identity",
    )?;

    let owner = OwnerReference::new(checked(HumanId::new("CX-EMP-000001"), "invalid owner id")?);

    let organizational_context = OrganizationalContext::new(ownership_path()?, owner.clone());

    let agent_definition = checked(
        AgentDefinition::new(AgentDefinitionSpec {
            identity,
            agent_uuid: checked(AgentUuid::new("CX-UUID-00000001"), "invalid agent uuid")?,
            agent_name: "Kernel Agent".to_owned(),
            agent_type: checked(AgentType::new("Supervisor"), "invalid agent type")?,
            agent_category: checked(AgentCategory::new("Operations"), "invalid agent category")?,
            owner: owner.clone(),
            organizational_context,
            runtime_reference: checked(
                AgentRuntimeReference::new("runtime.ref.000001"),
                "invalid runtime reference",
            )?,
        }),
        "invalid agent definition",
    )?;

    let runtime_entity = checked(
        RuntimeEntity::new(RuntimeEntitySpec {
            runtime_id: runtime_id.clone(),
            runtime_reference: checked(
                AgentRuntimeReference::new("runtime.ref.000001"),
                "invalid runtime reference",
            )?,
            enterprise_id: enterprise_id()?,
            ownership_path: ownership_path()?,
            health: RuntimeHealth::Healthy,
        }),
        "invalid runtime entity",
    )?;

    let capability = checked(
        CapabilityDescriptor::new(CapabilityDescriptorSpec {
            capability_id: checked(CapabilityId::new("CX-CAP-000001"), "invalid capability id")?,
            description: "Studio projection".to_owned(),
            dependencies: vec!["gateway".to_owned()],
            inputs: vec!["request".to_owned()],
            outputs: vec!["response".to_owned()],
            required_permissions: vec![permission_reference()?],
            governing_policies: vec![checked(
                PolicyId::new("CX-POL-000003"),
                "invalid policy id",
            )?],
        }),
        "invalid capability descriptor",
    )?;

    let lease = checked(
        LeaseRecord::issue(LeaseIssuanceSpec {
            lease_id: checked(LeaseId::new("CX-LEASE-000001"), "invalid lease id")?,
            runtime_id: runtime_id.clone(),
            agent_id: agent_id.clone(),
            issued_at: time_reference("2026-07-19T00:00:00Z")?,
            expires_at: time_reference("2026-07-19T01:00:00Z")?,
            supersedes_lease_id: None,
            evidence: "lease evidence".to_owned(),
        }),
        "invalid lease record",
    )?;

    let heartbeat = checked(
        HeartbeatRecord::observe(HeartbeatObservationSpec {
            heartbeat_id: checked(HeartbeatId::new("CX-HB-000001"), "invalid heartbeat id")?,
            runtime_id: runtime_id.clone(),
            agent_id: agent_id.clone(),
            recorded_at: time_reference("2026-07-19T00:00:00Z")?,
            fresh_until: time_reference("2026-07-19T00:30:00Z")?,
            reported_presence: PresenceState::Registered,
            reported_health: RuntimeHealth::Healthy,
            active_lease_id: Some(lease.lease_id().clone()),
            evidence: "heartbeat evidence".to_owned(),
        }),
        "invalid heartbeat record",
    )?;

    let registration = checked(
        AgentRegistration::new(AgentRegistrationSpec {
            agent: agent_definition,
            runtime: runtime_entity,
            supervisor: owner,
            capabilities: vec![capability],
            presence_state: PresenceState::Registered,
            health: RuntimeHealth::Healthy,
            registered_at: time_reference("2026-07-19T00:00:00Z")?,
            lease: Some(lease),
            last_heartbeat: Some(heartbeat),
        }),
        "invalid agent registration",
    )?;

    let mut registry = AgentRegistry::new();
    checked(
        registry.register(registration),
        "failed to register runtime agent",
    )?;

    let freshness_policy = checked(
        HeartbeatFreshnessPolicy::new(
            time_reference("2026-07-19T00:20:00Z")?,
            time_reference("2026-07-19T00:10:00Z")?,
            checked(
                StableVersion::new("freshness_version", "1.0.0"),
                "invalid freshness version",
            )?,
        ),
        "invalid heartbeat freshness policy",
    )?;

    checked(
        registry.runtime_snapshot(
            &agent_id,
            &time_reference("2026-07-19T00:30:00Z")?,
            &freshness_policy,
            None,
            None,
        ),
        "failed to create runtime snapshot",
    )
}

pub fn host_local_runtime_projection() -> Result<StudioRuntimeProjection, String> {
    let runtime_id = checked(RuntimeId::new("CX-RUN-000001"), "invalid runtime id")?;

    let execution_session_id = checked(
        ExecutionSessionId::new("execution.session-0001"),
        "invalid execution session id",
    )?;

    checked(
        StudioRuntimeProjection::new(
            runtime_id,
            vec![runtime_state_snapshot()?],
            vec![execution_session_id],
            host_local_studio_audit_reference()?,
        ),
        "invalid studio runtime projection",
    )
}

#[cfg(test)]
mod tests {
    use super::host_local_runtime_projection;

    #[test]
    fn builds_host_local_runtime_projection() {
        let projection = host_local_runtime_projection().expect("projection");

        assert_eq!(projection.selected_runtime_id().as_str(), "CX-RUN-000001");
        assert_eq!(projection.runtime_state_snapshots().len(), 1);
        assert_eq!(
            projection.runtime_state_snapshots()[0].agent_id().as_str(),
            "CX-AGT-000001"
        );
        assert_eq!(
            projection.current_execution_session_ids()[0].as_str(),
            "execution.session-0001"
        );
    }
}
