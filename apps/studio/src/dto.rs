use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub service: &'static str,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
pub struct TopViewResponse {
    pub enterprise_id: String,
    pub workspace_id: String,
    pub project_id: String,
    pub agents: Vec<String>,
    pub runtimes: Vec<String>,
    pub workflows: Vec<String>,
    pub tasks: Vec<String>,
    pub execution_sessions: Vec<String>,
    pub attention_state: String,
}

#[derive(Debug, Serialize)]
pub struct RuntimeSnapshotResponse {
    pub runtime_id: String,
    pub agent_id: String,
    pub presence: String,
    pub health: String,
    pub heartbeat_freshness: String,
    pub lease_assessment: String,
}

#[derive(Debug, Serialize)]
pub struct RuntimeResponse {
    pub selected_runtime_id: String,
    pub snapshots: Vec<RuntimeSnapshotResponse>,
    pub current_execution_session_ids: Vec<String>,
}
