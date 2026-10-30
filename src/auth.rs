use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};

use crate::{TarefasDb, errors::AppError, models::{CriarUsuarioRequest, Usuario}};

pub async fn criar_usuario(db: &TarefasDb, payload: CriarUsuarioRequest) -> Result<Usuario, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(payload.senha.as_bytes(), &salt)?
        .map(|h| h.to_string())?;
}

pub async fn autentica_usuario(db: &TarefasDb, email: &str, senha: &str) -> Result<Usuario, AppError> {

}