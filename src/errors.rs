use axum::{
    Json, http::{StatusCode}, response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NaoEncontrada,
    DadosInvalidos(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, mensagem) = match self {
            AppError::NaoEncontrada => {
                (StatusCode::NOT_FOUND, "Tarefa não encontrada".to_string())
            }
            AppError::DadosInvalidos(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
        };

        (status, Json(json!({"erro": mensagem}))).into_response()
    }
}
