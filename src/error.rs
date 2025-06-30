use actix_web::{HttpResponse, ResponseError};use thiserror::Error;
use crate::models::ApiResponse;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),

    #[error("Cryptographic error: {0}")]
    CryptoError(String),

    #[error("Token program error: {0}")]
    TokenError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let response = ApiResponse::<()>::error(self.to_string());
        
        match self {
            ServerError::ValidationError(_) => HttpResponse::BadRequest().json(response),
            ServerError::SolanaClientError(_) => HttpResponse::BadGateway().json(response),
            ServerError::CryptoError(_) => HttpResponse::BadRequest().json(response),
            ServerError::TokenError(_) => HttpResponse::BadRequest().json(response),
            ServerError::EncodingError(_) => HttpResponse::BadRequest().json(response),
            ServerError::InternalError => HttpResponse::InternalServerError().json(response),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;