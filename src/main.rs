mod behavior;
mod connection;
mod exchanges {
    pub mod bybit;
    pub mod exchange_factory;
    pub mod mexc;
}

// use chrono::{DateTime, Utc};
use dotenv::dotenv;
use exchanges::exchange_factory::ExchangeFactory;
// use quanta::Clock;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_type = env::var("API_TYPE").unwrap_or_else(|_| "unknown".to_string());
    let acces_key = env::var("ACCES_KEY").unwrap_or_else(|_| "unknown".to_string());
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| "unknown".to_string());
    println!("ACCES_KEY = {}", acces_key);
    println!("SECRET_KEY = {}", secret_key);

    let bybit = ExchangeFactory::new()
        .with_websocket_connection()
        .with_standard_behavior()
        .build_bybit();

    bybit.perform_task();

    // Создаем биржу Mexc с HTTP и альтернативной логикой
    let mexc = ExchangeFactory::new()
        .with_http_connection()
        .with_alternative_behavior()
        .build_mexc();

    mexc.perform_task();
    // // Определяем стратегию на основе переменной окружения
    // let strategy: Box<dyn ApiStrategy> = match api_type.as_str() {
    //     "wss" => Box::new(ApiWssStrategy),
    //     "http" => Box::new(ApiHttpStrategy),
    //     _ => panic!("Неизвестный тип API"),
    // };

    // // Создаем сервис с выбранной стратегией
    // let api_service = ApiService::new(strategy, acces_key, secret_key);

    // let clock = Clock::new();
    // let start_time = clock.now();
    // let start_system_time = std::time::SystemTime::now();

    // match api_service.get_api_restrictions().await {
    //     Ok(response) => {
    //         let end_time = clock.now();
    //         let end_system_time = std::time::SystemTime::now();
    //         let elapsed_time = end_time.duration_since(start_time);

    //         println!("API restrictions: {:?}", response);

    //         let start_time_formatted = start_system_time
    //             .duration_since(std::time::UNIX_EPOCH)
    //             .expect("Time went backwards")
    //             .as_secs_f64();
    //         let end_time_formatted = end_system_time
    //             .duration_since(std::time::UNIX_EPOCH)
    //             .expect("Time went backwards")
    //             .as_secs_f64();

    //         println!(
    //             "Start Time: {} - End Time: {} - Время выполнения: {} мкс",
    //             start_time_formatted,
    //             end_time_formatted,
    //             elapsed_time.as_micros()
    //         );
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //     }
    // }
}
