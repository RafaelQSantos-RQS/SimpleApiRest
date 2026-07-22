use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post}, Router};
use chrono::Utc;
use uuid::Uuid;
use std::cmp::Ordering;

mod models;
mod errors;

type TarefasDb = Arc<Mutex<HashMap<Uuid, models::Tarefa>>>;

#[derive(Clone)]
struct AppState {
    db: TarefasDb,
}

async fn listar_tarefas (
    State(state): State<AppState>,
    Query(params): Query<models::TarefaParametros>,
) -> Result<Json<Vec<models::Tarefa>>, errors::AppError> {
    let db = state.db.lock().map_err(|e| {
        errors::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    let mut tarefas: Vec<models::Tarefa> = db
        .values()
        .filter(
            |t| {
                params.concluida.is_none_or(|filtro| t.concluida == filtro)
            }
        )
        .cloned()
        .collect();

    tarefas.sort_by(|a,b| {
        match a.criada_em.cmp(&b.criada_em) {
            Ordering::Equal => a.id.cmp(&b.id),
            other => other
        }
    });
    
    let pagina = params.pagina.unwrap_or(1).max(1) as usize;
    let limite = params.limite.unwrap_or(10).max(1) as usize;

    let tarefas_paginadas: Vec<models::Tarefa> =  tarefas
        .into_iter()
        .skip((pagina-1)*limite)
        .take(limite)
        .collect();

    Ok(Json(tarefas_paginadas))
}

async fn buscar_tarefa (
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<models::Tarefa>, errors::AppError> {
    let db = state.db.lock().map_err(|e| {
        errors::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    db.get(&id)
        .cloned()
        .ok_or(errors::AppError::NaoEncontrada)
        .map(Json)
}

async fn criar_tarefa(
    State(state): State<AppState>,
    Json(payload): Json<models::CriarTarefaRequest>,
) -> Result<(StatusCode, Json<models::Tarefa>), errors::AppError> {
    if payload.titulo.trim().is_empty() {
        return Err(errors::AppError::DadosInvalidos(
            "O título não pode estar vazio".to_string(),
        ));
    }

    let agora = Utc::now();
    let tarefa = models::Tarefa {
        id: Uuid::new_v4(),
        titulo: payload.titulo,
        descricao: payload.descricao.unwrap_or_default(),
        concluida: false,
        criada_em: agora,
        atualizada_em: agora
    };

    let mut db = state.db.lock().map_err(|e| {
        errors::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    db.insert(tarefa.id, tarefa.clone());

    Ok((StatusCode::CREATED, Json(tarefa)))
}

async fn atualiza_tarefa(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<models::AtualizarTarefaRequest>
) -> Result<Json<models::Tarefa>, errors::AppError> {
    let mut db = state.db.lock().map_err(|e| {
        errors::AppError::Interno(format!("Erro ao acessar o banco: {}", e))
    })?;

    let tarefa = db.get_mut(&id).ok_or(errors::AppError::NaoEncontrada)?;

    if let Some(titulo) = payload.titulo {
        if titulo.trim().is_empty() {
            return Err(errors::AppError::DadosInvalidos(
                "O título não pode estar vazio".to_string(),
            ));
        }
        tarefa.titulo = titulo;
    }

    if let Some(descricao) = payload.descricao {
        tarefa.descricao = descricao;
    }

    if let Some(concluida) = payload.concluida {
        tarefa.concluida = concluida;
    }

    tarefa.atualizada_em = Utc::now();

    Ok(Json(tarefa.clone()))
}

async fn deletar_tarefa(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, errors::AppError> {
    let mut db = state.db.lock().map_err(
        |e| {
            errors::AppError::Interno(format!("Erro ao acessar o banco de dados: {}", e))
        }
    )?;

    db.remove(&id).ok_or(errors::AppError::NaoEncontrada)?;

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    // Initial state
    let shared_state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/tarefas", get(listar_tarefas))
        .route("/tarefas", post(criar_tarefa))
        .route(
            "/tarefas/{id}",
            get(buscar_tarefa)
            .put(atualiza_tarefa)
            .delete(deletar_tarefa)
        )
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

