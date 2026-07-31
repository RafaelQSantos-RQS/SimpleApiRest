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
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        AppError::Validacao(err)
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
        };

        (status, Json(mensagem)).into_response()
    }
}
