use crate::{
    behavior::behavior_strategy::{Behavior, BehaviorStrategy},
    connection::connection_strategy::ConnectionStrategy,
};

pub struct Mexc {
    name: String,
    connection: Box<dyn ConnectionStrategy>,
    behavior: Behavior,
}

impl Mexc {
    pub fn new(connection: Box<dyn ConnectionStrategy>, behavior: Behavior) -> Self {
        let name: String = "MEXC".to_string();
        Mexc {
            name,
            connection,
            behavior,
        }
    }

    pub async fn perform_task(&self) {
        // Подключение к Mexc
        self.connection.connect();
        println!("Работаем с {}", self.name);
        if let Err(e) = self.behavior.execute().await.await {
            eprintln!("Ошибка при выполнении задачи: {:?}", e);
        }
    }
}
