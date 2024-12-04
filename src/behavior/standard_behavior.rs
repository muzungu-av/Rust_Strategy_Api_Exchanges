use tokio::task::JoinHandle;

use crate::{
    connection::http_connection::HttpConnection, ErrorHandler, FirstRequestHandler, Handler,
    InitialHandler, SecondRequestHandler, SuccessHandler, ThirdRequestHandler,
};

use super::behavior_strategy::BehaviorStrategy;

use std::sync::Arc;

pub struct StandardBehavior {
    http_connection: Arc<HttpConnection>,
}

impl StandardBehavior {
    pub fn new(http_connection: Arc<HttpConnection>) -> Self {
        Self { http_connection }
    }
}

impl BehaviorStrategy for StandardBehavior {
    // вызывается из perform_task()
    async fn execute(&self) -> JoinHandle<()> {
        let client = Arc::new(self.http_connection.get_client().clone());
        // Создание асинхронной задачи для запуска цепочки обработчиков
        tokio::spawn(async move {
            let error_handler: Arc<dyn Handler> = Arc::new(ErrorHandler);
            let success_handler: Arc<dyn Handler> = Arc::new(SuccessHandler);

            let third_request: Arc<dyn Handler> = Arc::new(ThirdRequestHandler::new(
                Some(success_handler.clone()),
                error_handler.clone(),
                client.clone(),
                "https://jsonplaceholder.typicode.com/posts/3".to_string(),
            ));
            let second_request: Arc<dyn Handler> = Arc::new(SecondRequestHandler::new(
                Some(third_request),
                error_handler.clone(),
                client.clone(),
                "https://jsonplaceholder.typicode.com/posts/2".to_string(),
            ));
            let first_request: Arc<dyn Handler> = Arc::new(FirstRequestHandler::new(
                Some(second_request),
                error_handler.clone(),
                client.clone(),
                "https://jsonplaceholder.typicode.com/posts/1".to_string(),
            ));
            let initial: Arc<dyn Handler> = Arc::new(InitialHandler::new(Some(first_request)));

            // Запуск цепочки
            initial.handle(None).await;
        })
    }
}
