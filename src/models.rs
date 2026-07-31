use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: Uuid,
    pub titulo: String,
    pub descricao: String,
    pub concluida: bool,
    pub criada_em: DateTime<Utc>,
    pub atualizada_em: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CriarTarefaRequest {
    pub titulo: String,
    pub descricao: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AtualizarTarefaRequest {
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
