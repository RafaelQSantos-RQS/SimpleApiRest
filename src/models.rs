use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tarefa {
    pub id: Uuid,
    pub titulo: String,
    pub descricao: String,
    pub concluida: bool,
    pub criada_em: DateTime<Utc>,
    pub atualizada_em: DateTime<Utc>,
}

fn titulo_vazio(titulo: &str) -> Result<(), validator::ValidationError> {
    if titulo.trim().is_empty() {
        return Err(validator::ValidationError::new("titulo_vazio")
            .with_message("O título não pode estar vazio".into()));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct CriarTarefaRequest {
    #[validate(custom(function = "titulo_vazio"))]
    pub titulo: String,
    pub descricao: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AtualizarTarefaRequest {
    #[validate(custom(function = "titulo_vazio"))]
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub concluida: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TarefaParametros {
    pub concluida: Option<bool>,
    pub pagina: Option<i32>,
    pub limite: Option<i32>,
}
