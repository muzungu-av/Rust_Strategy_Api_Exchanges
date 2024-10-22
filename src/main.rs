mod behavior;
mod connection;

use behavior::alternative_behavior::AlternativeBehavior;
use behavior::standard_behavior::StandardBehavior;
use chrono::{DateTime, Utc};
use connection::http_connection::HttpConnection;
use connection::web_socket_connection::WebSocketConnection;
use dotenv::dotenv;
use quanta::Clock;
use std::env;

mod exchanges {
    pub mod bybit;
    pub mod mexc;
}

use exchanges::bybit::Bybit;
use exchanges::mexc::Mexc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_type = env::var("API_TYPE").unwrap_or_else(|_| "unknown".to_string());
    let acces_key = env::var("ACCES_KEY").unwrap_or_else(|_| "unknown".to_string());
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| "unknown".to_string());
    println!("ACCES_KEY = {}", acces_key);
    println!("SECRET_KEY = {}", secret_key);

    // Пример: создаем биржу Mexc с WebSocket подключением и стандартной логикой
    let mexc = Mexc::new(Box::new(WebSocketConnection), Box::new(StandardBehavior));

    mexc.perform_task();

    // Пример: создаем биржу Bybit с Http подключением и альтернативной логикой
    let bybit = Bybit::new(Box::new(HttpConnection), Box::new(AlternativeBehavior));

    bybit.perform_task();
}
