use crate::dto::{HealthResponse, TopViewResponse};
use crate::projection_factory::host_local_top_view;
use axum::{routing::get, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/studio/top-view", get(top_view))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "chela-x-studio",
        status: "ok",
    })
}

async fn top_view() -> Json<TopViewResponse> {
    Json(host_local_top_view())
}
