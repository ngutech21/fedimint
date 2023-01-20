use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::Path;
use axum::{routing::get, Json, Router};
use hyper::Method;
use mint_client::storage::StorageClient;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::create_client_module();

    client
        .retrieve_data(
            "1dcda432-54d8-479f-bd73-da38ac56c29f".to_string(),
            PathBuf::from("test123"),
        )
        .await?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::INFO, "startup");

    let addr = "[::]:8080".parse()?;
    event!(Level::INFO, "listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app(client)
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET]),
                )
                .into_make_service(),
        )
        .await?;
    Ok(())
}

fn app(storage_client: StorageClient) -> Router {
    Router::new()
        .route("/api/v1/storage/:id", get(get_storage))
        //.merge(SpaRouter::new("/", "frontend/dist/spa").index_file("index.html"))
        .with_state(Arc::new(storage_client))
        .layer(TraceLayer::new_for_http())
}

async fn get_storage(Path(_id): Path<i32>) -> Json<StorageItem> {
    let st = StorageItem("1234-22345-8779".to_string());
    Json(st)
}

#[derive(Serialize)]
pub struct StorageItem(pub String);
