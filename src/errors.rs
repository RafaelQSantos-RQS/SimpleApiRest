use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::{error, warn};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    NaoEncontrada,
    Validacao(ValidationErrors),
    Rejeicao(StatusCode, String),
    Interno(String),
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validacao(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NaoEncontrada,
            other => AppError::Interno(other.to_string()),
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        AppError::Rejeicao(value.status(), value.body_text())
    }
}

impl From<PathRejection> for AppError {
    fn from(value: PathRejection) -> Self {
        AppError::Rejeicao(value.status(), value.body_text())
    }
}

impl From<QueryRejection> for AppError {
    fn from(value: QueryRejection) -> Self {
        AppError::Rejeicao(value.status(), value.body_text())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensagem) = match &self {
            AppError::NaoEncontrada => {
                error!(?self, "Tarefa não encontrada");
                (
                    StatusCode::NOT_FOUND,
                    json!({"erro": "Tarefa não encontrada"}),
                )
            }
            AppError::Validacao(erros) => {
                warn!(?self, "Erro de validação");
                let detalhes: Vec<String> = erros
                    .field_errors()
                    .iter()
                    .flat_map(|(campo, erros)| {
                        erros.iter().map(move |e| {
                            e.message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| format!("{} inválido", campo))
                        })
                    })
                    .collect();
                (
                    StatusCode::BAD_REQUEST,
                    json!({"erro": "Dados inválidos", "detalhes": detalhes}),
                )
            }
            AppError::Interno(msg) => {
                error!(?self, "Erro interno {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({"erro": "Erro interno do servidor"}),
                )
            }
            AppError::Rejeicao(status, mensagem) => {
                warn!(?self, "Requisição rejeitada");
                (*status, json!({"erro": mensagem}))
            }
        };

        (status, Json(mensagem)).into_response()
    }
}
