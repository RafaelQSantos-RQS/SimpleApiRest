use std::{collections::HashMap, sync::Arc};

use simplerestapi::{AppState, criar_router};
use tokio::sync::RwLock;

mod config;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    config.init_logging();

    let state = AppState {
        db: Arc::new(RwLock::new(HashMap::new())),
    };
    let app = criar_router(state);

    let listener = tokio::net::TcpListener::bind(config.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
