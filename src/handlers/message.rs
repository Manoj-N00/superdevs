use actix_web::{post, web, HttpResponse};
use ed25519_dalek::{Keypair, PublicKey, Signature, SECRET_KEY_LENGTH, SIGNATURE_LENGTH, Signer, Verifier};
use crate::{
    models::{ApiResponse, SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse},
    error::{ServerResult, ServerError}
};

#[post("/message/sign")]
pub async fn sign_message(req: web::Json<SignMessageRequest>) -> ServerResult<HttpResponse> {
    let secret_bytes = bs58::decode(&req.secret)
        .into_vec()
        .map_err(|_| ServerError::ValidationError("Invalid secret key format".to_string()))?;

    if secret_bytes.len() != SECRET_KEY_LENGTH {
        return Err(ServerError::ValidationError("Invalid secret key length".to_string()));
    }

    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|e| ServerError::CryptoError(e.to_string()))?;
    
    let signature = keypair.sign(req.message.as_bytes());

    let response = SignMessageResponse {
        signature: base64::encode(signature.to_bytes()),
        public_key: bs58::encode(keypair.public.to_bytes()).into_string(),
        message: req.message.clone(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[post("/message/verify")]
pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> ServerResult<HttpResponse> {
    let pubkey_bytes = bs58::decode(&req.pubkey)
        .into_vec()
        .map_err(|_| ServerError::ValidationError("Invalid public key format".to_string()))?;

    let signature_bytes = base64::decode(&req.signature)
        .map_err(|_| ServerError::ValidationError("Invalid signature format".to_string()))?;

    if signature_bytes.len() != SIGNATURE_LENGTH {
        return Err(ServerError::ValidationError("Invalid signature length".to_string()));
    }

    let public_key = PublicKey::from_bytes(&pubkey_bytes)
        .map_err(|e| ServerError::CryptoError(e.to_string()))?;

    let signature = Signature::from_bytes(&signature_bytes)
        .map_err(|e| ServerError::CryptoError(e.to_string()))?;

    let is_valid = public_key
        .verify(req.message.as_bytes(), &signature)
        .is_ok();

    let response = VerifyMessageResponse {
        valid: is_valid,
        message: req.message.clone(),
        pubkey: req.pubkey.clone(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}