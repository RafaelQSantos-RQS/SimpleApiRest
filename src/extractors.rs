use crate::errors::AppError;
use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Query},
};
use serde::de::DeserializeOwned;

pub struct AppJson<T>(pub T);
impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

pub struct AppPath<T>(pub T);
impl<S, T> FromRequestParts<S> for AppPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(Path(value)) => Ok(Self(value)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

pub struct AppQuery<T>(pub T);
impl<S, T> FromRequestParts<S> for AppQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match Query::<T>::from_request_parts(parts, state).await {
            Ok(Query(value)) => Ok(Self(value)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}
