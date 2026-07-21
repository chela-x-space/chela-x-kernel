#![forbid(unsafe_code)]

mod dto;
mod mapper;
mod projection_factory;
mod routes;
mod runtime_projection_factory;
mod workflow_projection_factory;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router = routes::router();

    let address = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("CHELA-X Studio Host");
    println!("Listening on http://{address}");

    axum::serve(listener, app).await?;

    Ok(())
}
