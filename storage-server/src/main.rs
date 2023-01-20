use std::sync::Arc;

use axum::extract::{Path, State};
use axum::{routing::get, Router};
use hyper::Method;
use mint_client::storage::StorageClient;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = client::create_client_module();

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

async fn get_storage(
    Path(id): Path<String>,
    State(storage_client): State<Arc<StorageClient>>,
) -> Vec<u8> {
    storage_client.retrieve_data_raw(id).await.unwrap()
    // FIXME error handling
}
