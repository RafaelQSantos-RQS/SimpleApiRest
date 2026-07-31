use chrono::Utc;
use tracing::instrument;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{errors, models};

type TarefasDb = Arc<RwLock<HashMap<Uuid, models::Tarefa>>>;

#[instrument(skip(db), fields(titulo = %payload.titulo))]
pub async fn criar_tarefa(
    db: &TarefasDb,
    payload: models::CriarTarefaRequest,
) -> Result<models::Tarefa, errors::AppError> {
    let agora = Utc::now();
    let tarefa_a_ser_criada = models::Tarefa {
        id: Uuid::new_v4(),
        titulo: payload.titulo,
        descricao: payload.descricao.unwrap_or_default(),
        concluida: false,
        criada_em: agora,
        atualizada_em: agora,
    };

    let mut db = db.write().await;

    db.insert(tarefa_a_ser_criada.id, tarefa_a_ser_criada.clone());

    Ok(tarefa_a_ser_criada)
}

#[instrument(skip(db), fields(id = %id))]
pub async fn atualizar_tarefa(
    db: &TarefasDb,
    id: Uuid,
    payload: models::AtualizarTarefaRequest,
) -> Result<models::Tarefa, errors::AppError> {
    let mut db = db.write().await;

    // Buscando se o ID existe no banco
    let tarefa = db.get_mut(&id).ok_or(errors::AppError::NaoEncontrada)?;

    if let Some(titulo) = payload.titulo {
        tarefa.titulo = titulo;
    }

    if let Some(descricao) = payload.descricao {
        tarefa.descricao = descricao;
    }

    if let Some(concluida) = payload.concluida {
        tarefa.concluida = concluida;
    }

    tarefa.atualizada_em = Utc::now();

    Ok(tarefa.clone())
}

#[instrument(skip(db), fields(id = %id))]
pub async fn deletar_tarefa(db: &TarefasDb, id: Uuid) -> Result<(), errors::AppError> {
    let mut db = db.write().await;

    // Apagando o registro caso o mesmo exista
    db.remove(&id).ok_or(errors::AppError::NaoEncontrada)?;
    Ok(())
}

#[instrument(skip(db), fields(id = %id))]
pub async fn buscar_tarefa(db: &TarefasDb, id: Uuid) -> Result<models::Tarefa, errors::AppError> {
    let db = db.read().await;

    let tarefa = db.get(&id).ok_or(errors::AppError::NaoEncontrada)?;

    Ok(tarefa.clone())
}

#[instrument(skip(db), fields(pagina = parametros.pagina, limite = parametros.limite, concluida = parametros.concluida))]
pub async fn listar_tarefas(
    db: &TarefasDb,
    parametros: models::TarefaParametros,
) -> Result<Vec<models::Tarefa>, errors::AppError> {
    let db = db.read().await;

    let mut lista_de_tarefas: Vec<models::Tarefa> = db
        .values()
        .filter(|t| {
            parametros
                .concluida
                .is_none_or(|filtro| t.concluida == filtro)
        })
        .cloned()
        .collect();

    lista_de_tarefas.sort_by(|a, b| match a.criada_em.cmp(&b.criada_em) {
        Ordering::Equal => a.id.cmp(&b.id),
        other => other,
    });

    let pagina = parametros.pagina.unwrap_or(1) as usize;
    let limite = parametros.limite.unwrap_or(10) as usize;

    let lista_de_tarefas_paginas: Vec<models::Tarefa> = lista_de_tarefas
        .into_iter()
        .skip((pagina - 1) * limite)
        .take(limite)
        .collect();

    Ok(lista_de_tarefas_paginas)
}
