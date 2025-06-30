use actix_web::{web, App, HttpServer, middleware};
use log::info;

mod handlers;
mod models;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting Solana HTTP server");
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/")
                    .service(handlers::keypair::generate_keypair)
                    .service(handlers::token::create_token)
                    .service(handlers::token::mint_token)
                    .service(handlers::message::sign_message)
                    .service(handlers::message::verify_message)
                    .service(handlers::transfer::send_sol)
                    .service(handlers::transfer::send_token)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}