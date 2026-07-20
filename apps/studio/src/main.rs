#![forbid(unsafe_code)]

use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Debug, Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct TopViewResponse {
    enterprise_id: &'static str,
    workspace_id: &'static str,
    project_id: &'static str,
    agents: Vec<&'static str>,
    runtimes: Vec<&'static str>,
    workflows: Vec<&'static str>,
    tasks: Vec<&'static str>,
    execution_sessions: Vec<&'static str>,
    attention_state: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/studio/top-view", get(top_view));

    let address = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("CHELA-X Studio Host");
    println!("Listening on http://{address}");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "chela-x-studio",
        status: "ok",
    })
}

async fn top_view() -> Json<TopViewResponse> {
    Json(TopViewResponse {
        enterprise_id: "CX-ENT-000001",
        workspace_id: "CX-WS-000001",
        project_id: "CX-PROJ-000001",
        agents: vec!["CX-AGT-000001"],
        runtimes: vec!["CX-RUN-000001"],
        workflows: vec!["CX-WF-000001"],
        tasks: vec!["CX-TASK-000001"],
        execution_sessions: vec!["execution.session-0001"],
        attention_state: "NeedsAttention",
    })
}
