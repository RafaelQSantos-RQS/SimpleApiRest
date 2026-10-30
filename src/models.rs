use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Tarefa {
    pub id: Uuid,
    pub titulo: String,
    pub descricao: Option<String>,
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
    pub pagina: Option<i64>,
    pub limite: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Paginacao {
    pub pagina: i64,
    pub limite: i64,
    pub total: i64,
    pub tem_mais: bool
}

#[derive(Debug, Serialize)]
pub struct PaginaDeTarefas {
    pub dados: Vec<Tarefa>,
    pub paginacao: Paginacao
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub criada_em: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UsuarioPublico {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub criada_em: DateTime<Utc>
}

impl From<Usuario> for UsuarioPublico {
    fn from(u: Usuario) -> Self {
        Self {
            id: u.id,
            nome: u.nome,
            email: u.email,
            criada_em: u.criada_em
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CriarUsuarioRequest {
    pub nome: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 64))]
    pub senha: String
}