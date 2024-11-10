mod behavior;
mod connection;
mod exchanges {
    pub mod bybit;
    pub mod exchange_factory;
    pub mod mexc;
}
mod request_machine;

use dotenv::dotenv;
use exchanges::exchange_factory::ExchangeFactory;
use request_machine::*;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_type = env::var("API_CONNECTION_TYPE").unwrap_or_else(|_| "unknown".to_string());
    let acces_key = env::var("ACCES_KEY").unwrap_or_else(|_| "unknown".to_string());
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| "unknown".to_string());
    let behavior = env::var("BEHAVIOR").unwrap_or_else(|_| "unknown".to_string());

    println!("ACCES_KEY = {}", acces_key);
    println!("SECRET_KEY = {}", secret_key);

    let mexc = ExchangeFactory::new()
        .connection(&api_type)
        .behavior(&behavior)
        .build_mexc();

    mexc.perform_task().await;
}
