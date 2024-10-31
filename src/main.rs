mod behavior;
mod connection;
mod exchanges {
    pub mod bybit;
    pub mod exchange_factory;
    pub mod mexc;
}
mod request_machine;

use std::sync::Arc;

use request_machine::*;
use tokio::main;

#[tokio::main]
async fn main() {
    let error_handler: Arc<dyn Handler> = Arc::new(ErrorHandler);
    let success_handler: Arc<dyn Handler> = Arc::new(SuccessHandler);

    let third_request: Arc<dyn Handler> = Arc::new(ThirdRequestHandler::new(
        Some(success_handler.clone()),
        error_handler.clone(),
    ));
    let second_request: Arc<dyn Handler> = Arc::new(SecondRequestHandler::new(
        Some(third_request),
        error_handler.clone(),
    ));
    let first_request: Arc<dyn Handler> = Arc::new(FirstRequestHandler::new(
        Some(second_request),
        error_handler.clone(),
    ));
    let initial: Arc<dyn Handler> = Arc::new(InitialHandler::new(Some(first_request)));

    // Запуск цепочки
    initial.handle().await;
}
