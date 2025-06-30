use actix_web::{post, web, HttpResponse};
use solana_program::{system_instruction, pubkey::Pubkey};
use spl_token::instruction as token_instruction;
use spl_token::id as token_program_id;
use spl_associated_token_account::get_associated_token_address;
use base64::Engine;

use crate::{
    models::{ApiResponse, SendSolRequest, SendTokenRequest, InstructionResponse, AccountMeta},
    error::{ServerResult, ServerError}
};

#[post("/send/sol")]
pub async fn send_sol(req: web::Json<SendSolRequest>) -> ServerResult<HttpResponse> {
    let from_pubkey = req.from.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid sender address".to_string()))?;
    
    let to_pubkey = req.to.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid recipient address".to_string()))?;

    if req.lamports == 0 {
        return Err(ServerError::ValidationError("Amount must be greater than 0".to_string()));
    }

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| AccountMeta {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }).collect(),
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[post("/send/token")]
pub async fn send_token(req: web::Json<SendTokenRequest>) -> ServerResult<HttpResponse> {
    let destination = req.destination.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid destination address".to_string()))?;
    
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid mint address".to_string()))?;
    
    let owner = req.owner.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid owner address".to_string()))?;

    if req.amount == 0 {
        return Err(ServerError::ValidationError("Amount must be greater than 0".to_string()));
    }

    // ✅ Correct usage
    let owner_ata = get_associated_token_address(&owner, &mint);
    let destination_ata = get_associated_token_address(&destination, &mint);

    let instruction = token_instruction::transfer(
        &token_program_id(),
        &owner_ata,
        &destination_ata,
        &owner,
        &[],
        req.amount,
    ).map_err(|e| ServerError::TokenError(e.to_string()))?;

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| AccountMeta {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }).collect(),
        instruction_data: base64::engine::general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
