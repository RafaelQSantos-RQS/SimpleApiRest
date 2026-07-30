use std::{collections::HashMap, sync::Arc};

use simplerestapi::{AppState, criar_router};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let state = AppState{
        db: Arc::new(RwLock::new(HashMap::new()))
    };
    let app = criar_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

