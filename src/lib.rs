use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post}, Router};
use uuid::Uuid;

use crate::services::{atualizar_tarefa, buscar_tarefa, criar_tarefa, deletar_tarefa, listar_tarefas};

mod models;
mod errors;
mod services;

pub type TarefasDb = Arc<RwLock<HashMap<Uuid, models::Tarefa>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: TarefasDb,
}

pub async fn listar_tarefas_handler (
    State(state): State<AppState>,
    Query(params): Query<models::TarefaParametros>,
) -> Result<Json<Vec<models::Tarefa>>, errors::AppError> {
    let tarefas_paginadas = listar_tarefas(&state.db, params).await?;
    Ok(Json(tarefas_paginadas))
}

pub async fn buscar_tarefa_handler (
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<models::Tarefa>, errors::AppError> {
    let tarefa_buscada = buscar_tarefa(&state.db, id).await?;

    Ok(Json(tarefa_buscada))
}

pub async fn criar_tarefa_handler(
    State(state): State<AppState>,
    Json(payload): Json<models::CriarTarefaRequest>,
) -> Result<(StatusCode, Json<models::Tarefa>), errors::AppError> {
    let tarefa_criada = criar_tarefa(&state.db, payload).await?;

    Ok((StatusCode::CREATED, Json(tarefa_criada)))
}

pub async fn atualiza_tarefa_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<models::AtualizarTarefaRequest>
) -> Result<Json<models::Tarefa>, errors::AppError> {
    let tarefa_atualizada = atualizar_tarefa(&state.db, id, payload).await?;
    Ok(Json(tarefa_atualizada))
}

pub async fn deletar_tarefa_handler (
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, errors::AppError> {
    deletar_tarefa(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn criar_router(
    state: AppState
) -> Router{
    Router::new()
        .route("/tarefas", get(listar_tarefas_handler))
        .route("/tarefas", post(criar_tarefa_handler))
        .route(
            "/tarefas/{id}",
            get(buscar_tarefa_handler)
            .put(atualiza_tarefa_handler)
            .delete(deletar_tarefa_handler)
        )
        .with_state(state)
}