use crate::dto::TopViewResponse;

/// Host-local Top View data source.
///
/// This preserves the current HTTP behavior while isolating sample data from
/// transport code. A later step will replace this function with construction
/// and mapping of `kernel_studio::StudioTopViewProjection`.
pub fn host_local_top_view() -> TopViewResponse {
    TopViewResponse {
        enterprise_id: "CX-ENT-000001".to_owned(),
        workspace_id: "CX-WS-000001".to_owned(),
        project_id: "CX-PROJ-000001".to_owned(),
        agents: vec!["CX-AGT-000001".to_owned()],
        runtimes: vec!["CX-RUN-000001".to_owned()],
        workflows: vec!["CX-WF-000001".to_owned()],
        tasks: vec!["CX-TASK-000001".to_owned()],
        execution_sessions: vec!["execution.session-0001".to_owned()],
        attention_state: "NeedsAttention".to_owned(),
    }
}
