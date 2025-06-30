use actix_web::{post, web, HttpResponse};
use solana_program::{system_program, pubkey::Pubkey};
use solana_sdk::signer::Signer;
use spl_token::instruction as token_instruction;
use crate::{
    models::{ApiResponse, CreateTokenRequest, MintTokenRequest, InstructionResponse, AccountMeta},
    error::{ServerResult, ServerError}
};

#[post("/token/create")]
pub async fn create_token(req: web::Json<CreateTokenRequest>) -> ServerResult<HttpResponse> {
    let mint_authority = req.mint_authority.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid mint authority pubkey".to_string()))?;
    
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid mint pubkey".to_string()))?;

    let instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        None,
        req.decimals
    ).map_err(|e| ServerError::TokenError(e.to_string()))?;

    let response = InstructionResponse {
        program_id: instruction.program_id.to_string(),
        accounts: instruction.accounts.iter().map(|acc| AccountMeta {
            pubkey: acc.pubkey.to_string(),
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        }).collect(),
        instruction_data: base64::encode(instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

#[post("/token/mint")]
pub async fn mint_token(req: web::Json<MintTokenRequest>) -> ServerResult<HttpResponse> {
    let mint = req.mint.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid mint address".to_string()))?;
    
    let destination = req.destination.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid destination address".to_string()))?;
    
    let authority = req.authority.parse::<Pubkey>()
        .map_err(|_| ServerError::ValidationError("Invalid authority address".to_string()))?;

    let instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination,
        &authority,
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
        instruction_data: base64::encode(instruction.data),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}