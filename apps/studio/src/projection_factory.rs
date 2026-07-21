use kernel_domain::{
    AgentId, AuditEvidenceId, CorrelationId, EnterpriseId, EventTraceReference, ExecutionSessionId,
    OwnershipPath, ProjectId, RuntimeId, TaskInstanceId, TaskInstanceReference, WorkflowId,
    WorkspaceId,
};
use kernel_gateway::GatewayAuditReference;
use kernel_studio::{StudioAttentionState, StudioAuditReference, StudioTopViewProjection};

pub fn host_local_studio_audit_reference() -> Result<StudioAuditReference, String> {
    let gateway_audit_reference = GatewayAuditReference::new(
        EventTraceReference::new("gateway.audit.trace.000001")
            .map_err(|error| format!("invalid gateway trace: {error:?}"))?,
        Some(
            CorrelationId::new("CX-COR-000001")
                .map_err(|error| format!("invalid correlation id: {error:?}"))?,
        ),
        vec![AuditEvidenceId::new("CX-AUD-000001")
            .map_err(|error| format!("invalid audit evidence id: {error:?}"))?],
    )
    .map_err(|error| format!("invalid gateway audit reference: {error:?}"))?;

    StudioAuditReference::new(
        EventTraceReference::new("studio.audit.trace.000001")
            .map_err(|error| format!("invalid studio trace: {error:?}"))?,
        Some(
            CorrelationId::new("CX-COR-000001")
                .map_err(|error| format!("invalid correlation id: {error:?}"))?,
        ),
        vec![AuditEvidenceId::new("CX-AUD-000001")
            .map_err(|error| format!("invalid audit evidence id: {error:?}"))?],
        Some(gateway_audit_reference),
    )
    .map_err(|error| format!("invalid studio audit reference: {error:?}"))
}

pub fn host_local_top_view_projection() -> Result<StudioTopViewProjection, String> {
    let ownership_path = OwnershipPath::new(
        EnterpriseId::new("CX-ENT-000001")
            .map_err(|error| format!("invalid enterprise id: {error:?}"))?,
        Some(
            WorkspaceId::new("CX-WS-000001")
                .map_err(|error| format!("invalid workspace id: {error:?}"))?,
        ),
        Some(
            ProjectId::new("CX-PROJ-000001")
                .map_err(|error| format!("invalid project id: {error:?}"))?,
        ),
        None,
    )
    .map_err(|error| format!("invalid ownership path: {error:?}"))?;

    let studio_audit_reference = host_local_studio_audit_reference()?;

    StudioTopViewProjection::new(
        ownership_path,
        vec![AgentId::new("CX-AGT-000001")
            .map_err(|error| format!("invalid agent id: {error:?}"))?],
        vec![RuntimeId::new("CX-RUN-000001")
            .map_err(|error| format!("invalid runtime id: {error:?}"))?],
        vec![WorkflowId::new("CX-WF-000001")
            .map_err(|error| format!("invalid workflow id: {error:?}"))?],
        vec![TaskInstanceReference::new(
            TaskInstanceId::new("CX-TASK-000001")
                .map_err(|error| format!("invalid task instance id: {error:?}"))?,
        )],
        vec![ExecutionSessionId::new("execution.session-0001")
            .map_err(|error| format!("invalid execution session id: {error:?}"))?],
        StudioAttentionState::NeedsAttention,
        studio_audit_reference,
    )
    .map_err(|error| format!("invalid studio top-view projection: {error:?}"))
}

#[cfg(test)]
mod tests {
    use super::host_local_top_view_projection;

    #[test]
    fn builds_host_local_projection_with_frozen_studio_contracts() {
        let projection = host_local_top_view_projection().expect("projection");

        assert_eq!(
            projection.ownership_path().enterprise_id().as_str(),
            "CX-ENT-000001"
        );
        assert_eq!(projection.agent_ids()[0].as_str(), "CX-AGT-000001");
        assert_eq!(projection.runtime_ids()[0].as_str(), "CX-RUN-000001");
        assert_eq!(projection.workflow_ids()[0].as_str(), "CX-WF-000001");
        assert_eq!(
            projection.task_instance_references()[0]
                .task_instance_id()
                .as_str(),
            "CX-TASK-000001"
        );
        assert_eq!(
            projection.execution_session_ids()[0].as_str(),
            "execution.session-0001"
        );
    }
}
