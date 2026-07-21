use crate::dto::{
    RuntimeResponse, RuntimeSnapshotResponse, TopViewResponse, WorkflowResponse,
    WorkflowSnapshotResponse,
};
use kernel_studio::{StudioRuntimeProjection, StudioTopViewProjection, StudioWorkflowProjection};

pub fn top_view_response(projection: &StudioTopViewProjection) -> TopViewResponse {
    let ownership_path = projection.ownership_path();

    TopViewResponse {
        enterprise_id: ownership_path.enterprise_id().as_str().to_owned(),
        workspace_id: ownership_path
            .workspace_id()
            .map_or_else(String::new, |id| id.as_str().to_owned()),
        project_id: ownership_path
            .project_id()
            .map_or_else(String::new, |id| id.as_str().to_owned()),
        agents: projection
            .agent_ids()
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
        runtimes: projection
            .runtime_ids()
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
        workflows: projection
            .workflow_ids()
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
        tasks: projection
            .task_instance_references()
            .iter()
            .map(|reference| reference.task_instance_id().as_str().to_owned())
            .collect(),
        execution_sessions: projection
            .execution_session_ids()
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
        attention_state: format!("{:?}", projection.attention_state()),
    }
}

pub fn runtime_response(projection: &StudioRuntimeProjection) -> RuntimeResponse {
    RuntimeResponse {
        selected_runtime_id: projection.selected_runtime_id().as_str().to_owned(),
        snapshots: projection
            .runtime_state_snapshots()
            .iter()
            .map(|snapshot| RuntimeSnapshotResponse {
                runtime_id: snapshot.runtime_id().as_str().to_owned(),
                agent_id: snapshot.agent_id().as_str().to_owned(),
                presence: format!("{:?}", snapshot.presence()),
                health: format!("{:?}", snapshot.health()),
                heartbeat_freshness: format!("{:?}", snapshot.heartbeat_freshness()),
                lease_assessment: format!("{:?}", snapshot.lease_assessment()),
            })
            .collect(),
        current_execution_session_ids: projection
            .current_execution_session_ids()
            .iter()
            .map(|id| id.as_str().to_owned())
            .collect(),
    }
}

pub fn workflow_response(projection: &StudioWorkflowProjection) -> WorkflowResponse {
    let snapshot = projection.workflow_state_snapshot();

    WorkflowResponse {
        snapshot: WorkflowSnapshotResponse {
            workflow_id: snapshot.workflow_id().as_str().to_owned(),
            lifecycle: format!("{:?}", snapshot.lifecycle()),
            definition_version: format!("{:?}", snapshot.definition_version()),
            sequence: format!("{:?}", snapshot.sequence()),
            current_step: projection
                .current_step_reference()
                .map(|step| step.as_str().to_owned()),
            completed_steps: projection
                .completed_step_references()
                .iter()
                .map(|step| step.as_str().to_owned())
                .collect(),
            blocked_steps: projection
                .blocked_step_references()
                .iter()
                .map(|step| step.as_str().to_owned())
                .collect(),
            task_instances: projection
                .task_instance_references()
                .iter()
                .map(|reference| reference.task_instance_id().as_str().to_owned())
                .collect(),
            execution_sessions: projection
                .execution_session_ids()
                .iter()
                .map(|id| id.as_str().to_owned())
                .collect(),
            workflow_failure_code: projection
                .workflow_failure_code()
                .map(|code| code.as_str().to_owned()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{runtime_response, top_view_response, workflow_response};
    use crate::projection_factory::host_local_top_view_projection;
    use crate::runtime_projection_factory::host_local_runtime_projection;
    use crate::workflow_projection_factory::host_local_workflow_projection;

    #[test]
    fn maps_studio_top_view_projection_to_http_response() {
        let projection = host_local_top_view_projection().expect("projection");
        let response = top_view_response(&projection);

        assert_eq!(response.enterprise_id, "CX-ENT-000001");
        assert_eq!(response.workspace_id, "CX-WS-000001");
        assert_eq!(response.project_id, "CX-PROJ-000001");
        assert_eq!(response.agents, ["CX-AGT-000001"]);
        assert_eq!(response.runtimes, ["CX-RUN-000001"]);
        assert_eq!(response.workflows, ["CX-WF-000001"]);
        assert_eq!(response.tasks, ["CX-TASK-000001"]);
        assert_eq!(response.execution_sessions, ["execution.session-0001"]);
        assert_eq!(response.attention_state, "NeedsAttention");
    }

    #[test]
    fn maps_studio_runtime_projection_to_http_response() {
        let projection = host_local_runtime_projection().expect("projection");
        let response = runtime_response(&projection);

        assert_eq!(response.selected_runtime_id, "CX-RUN-000001");
        assert_eq!(response.snapshots.len(), 1);
        assert_eq!(response.snapshots[0].runtime_id, "CX-RUN-000001");
        assert_eq!(response.snapshots[0].agent_id, "CX-AGT-000001");
        assert_eq!(response.snapshots[0].presence, "Registered");
        assert_eq!(response.snapshots[0].health, "Critical");
        assert_eq!(
            response.current_execution_session_ids,
            ["execution.session-0001"]
        );
    }

    #[test]
    fn maps_studio_workflow_projection_to_http_response() {
        let projection = host_local_workflow_projection().expect("projection");
        let response = workflow_response(&projection);

        assert_eq!(response.snapshot.workflow_id, "CX-WF-000001");
        assert_eq!(response.snapshot.lifecycle, "Running");
        assert_eq!(
            response.snapshot.current_step.as_deref(),
            Some("step.current")
        );
        assert_eq!(response.snapshot.completed_steps, ["step.done"]);
        assert_eq!(response.snapshot.blocked_steps, ["step.blocked"]);
        assert_eq!(response.snapshot.task_instances, ["CX-TASK-000001"]);
        assert_eq!(
            response.snapshot.execution_sessions,
            ["execution.session-0001"]
        );
        assert_eq!(
            response.snapshot.workflow_failure_code.as_deref(),
            Some("WF_INVALID_TRANSITION")
        );
    }
}
