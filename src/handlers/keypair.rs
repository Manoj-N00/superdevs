use actix_web::{post, HttpResponse};
use solana_sdk::signer::{keypair::Keypair, Signer};
use crate::{models::{ApiResponse, KeypairResponse}, error::ServerResult};

#[post("/keypair")]
pub async fn generate_keypair() -> ServerResult<HttpResponse> {
    let keypair = Keypair::new();
    
    let response = KeypairResponse {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}