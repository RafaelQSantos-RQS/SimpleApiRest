use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::TarefasDb;
use crate::{errors, models};

#[instrument(skip(db), fields(titulo = %payload.titulo))]
pub async fn criar_tarefa(
    db: &TarefasDb,
    payload: models::CriarTarefaRequest,
) -> Result<models::Tarefa, errors::AppError> {
    let agora = Utc::now();
    let tarefa = models::Tarefa {
        id: Uuid::new_v4(),
        titulo: payload.titulo,
        descricao: payload.descricao.unwrap_or_default(),
        concluida: false,
        criada_em: agora,
        atualizada_em: agora,
    };

    sqlx::query(
        "INSERT INTO tarefas (id, titulo, descricao, concluida, criada_em, atualizada_em)
        VALUES (?,?,?,?,?,?)",
    )
    .bind(tarefa.id)
    .bind(&tarefa.titulo)
    .bind(&tarefa.descricao)
    .bind(tarefa.concluida)
    .bind(tarefa.criada_em)
    .bind(tarefa.atualizada_em)
    .execute(db)
    .await?;

    Ok(tarefa)
}

#[instrument(skip(db), fields(id = %id))]
pub async fn atualizar_tarefa(
    db: &TarefasDb,
    id: Uuid,
    payload: models::AtualizarTarefaRequest,
) -> Result<models::Tarefa, errors::AppError> {
    let tarefa_atualizada = sqlx::query_as::<_, models::Tarefa>(
        "UPDATE tarefas SET
            titulo = COALESCE(?, titulo),
            descricao = COALESCE(?, descricao),
            concluida = COALESCE(?, concluida),
            atualizada_em = ?
        WHERE id = ?
        RETURNING *
        ",
    )
    .bind(payload.titulo)
    .bind(payload.descricao)
    .bind(payload.concluida)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(tarefa_atualizada)
}

#[instrument(skip(db), fields(id = %id))]
pub async fn deletar_tarefa(db: &TarefasDb, id: Uuid) -> Result<(), errors::AppError> {
    let resultado = sqlx::query("DELETE FROM tarefas WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(errors::AppError::NaoEncontrada);
    }

    Ok(())
}

#[instrument(skip(db), fields(id = %id))]
pub async fn buscar_tarefa(db: &TarefasDb, id: Uuid) -> Result<models::Tarefa, errors::AppError> {
    let tarefa = sqlx::query_as::<_, models::Tarefa>("select * from tarefas where id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(tarefa)
}

#[instrument(skip(db), fields(pagina = parametros.pagina, limite = parametros.limite, concluida = parametros.concluida))]
pub async fn listar_tarefas(
    db: &TarefasDb,
    parametros: models::TarefaParametros,
) -> Result<Vec<models::Tarefa>, errors::AppError> {
    let pagina = parametros.pagina.unwrap_or(1).max(1) as usize;
    let limite = parametros.limite.unwrap_or(10).clamp(1, 100) as usize;

    let lista_de_tarefas = sqlx::query_as::<_, models::Tarefa>(
        "
        SELECT
            *
        FROM tarefas
        WHERE (? is NULL OR concluida = ?)
        ORDER BY criada_em, id ASC
        LIMIT ? OFFSET ?
        ",
    )
    .bind(parametros.concluida)
    .bind(parametros.concluida)
    .bind(limite as i64)
    .bind(((pagina - 1) * limite) as i64)
    .fetch_all(db)
    .await?;

    Ok(lista_de_tarefas)
}
