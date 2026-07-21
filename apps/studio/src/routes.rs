use crate::dto::{HealthResponse, RuntimeResponse, TopViewResponse};
use crate::mapper::{runtime_response, top_view_response};
use crate::projection_factory::host_local_top_view_projection;
use crate::runtime_projection_factory::host_local_runtime_projection;
use axum::{http::StatusCode, routing::get, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/studio/top-view", get(top_view))
        .route("/api/studio/runtime", get(runtime))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "chela-x-studio",
        status: "ok",
    })
}

async fn top_view() -> Result<Json<TopViewResponse>, (StatusCode, String)> {
    let projection = host_local_top_view_projection().map_err(|detail| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to build top-view projection: {detail}"),
        )
    })?;

    Ok(Json(top_view_response(&projection)))
}

async fn runtime() -> Result<Json<RuntimeResponse>, (StatusCode, String)> {
    let projection = host_local_runtime_projection().map_err(|detail| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to build runtime projection: {detail}"),
        )
    })?;

    Ok(Json(runtime_response(&projection)))
}
