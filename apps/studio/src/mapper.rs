use crate::dto::TopViewResponse;
use kernel_studio::StudioTopViewProjection;

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

#[cfg(test)]
mod tests {
    use super::top_view_response;
    use crate::projection_factory::host_local_top_view_projection;

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
}
