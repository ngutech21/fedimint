use std::sync::Arc;

use axum::extract::{Multipart, Path, State};
use axum::routing::post;
use axum::Json;
use axum::{routing::get, Router};
use axum_extra::routing::SpaRouter;
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
                        .allow_methods([Method::GET, Method::POST]),
                )
                .into_make_service(),
        )
        .await?;
    Ok(())
}

fn app(storage_client: StorageClient) -> Router {
    Router::new()
        .merge(SpaRouter::new("/", "assets").index_file("index.html"))
        .route("/api/v1/storage/:id", get(get_storage))
        .route("/api/v1/storage", post(post_storage))
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

async fn post_storage(
    State(storage_client): State<Arc<StorageClient>>,
    mut multipart: Multipart,
) -> Json<String> {
    // FIXME error handling
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let bytes = data.into_iter().collect::<Vec<u8>>();
        let key = storage_client.store_data_raw(bytes).await.unwrap();
        event!(Level::INFO, "Filename: {} Key: {}", file_name, key);

        return Json(key);
    }
    Json("Invalid request".to_string())
}
