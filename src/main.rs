use simplerestapi::{AppState, criar_router};
use sqlx::SqlitePool;

mod config;
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    config.init_logging();

    let pool = SqlitePool::connect("sqlite:tarefas.db?mode=rwc")
        .await
        .expect("Falha ao conectar no banco");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Falha ao rodar as migrations");

    let state = AppState { db: pool };

    let app = criar_router(state);

    let listener = tokio::net::TcpListener::bind(config.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
