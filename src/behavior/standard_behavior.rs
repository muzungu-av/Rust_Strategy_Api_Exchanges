use tokio::task::JoinHandle;

use crate::{
    ErrorHandler, FirstRequestHandler, Handler, InitialHandler, SecondRequestHandler,
    SuccessHandler, ThirdRequestHandler,
};

use super::behavior_strategy::BehaviorStrategy;

use std::sync::Arc;

pub struct StandardBehavior;

impl BehaviorStrategy for StandardBehavior {
    async fn execute(&self) -> JoinHandle<()> {
        // Создание асинхронной задачи для запуска цепочки обработчиков
        tokio::spawn(async {
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
            initial.handle(None).await;
        })
    }
}
