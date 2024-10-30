use crate::{
    behavior::behavior_strategy::BehaviorStrategy,
    connection::connection_strategy::ConnectionStrategy,
};

pub struct Mexc {
    name: String,
    connection: Box<dyn ConnectionStrategy>,
    behavior: Box<dyn BehaviorStrategy>,
}

impl Mexc {
    pub fn new(
        connection: Box<dyn ConnectionStrategy>,
        behavior: Box<dyn BehaviorStrategy>,
    ) -> Self {
        let name: String = "MEXC".to_string();
        Mexc {
            name,
            connection,
            behavior,
        }
    }

    pub fn perform_task(&self) {
        // Подключение к Mexc
        self.connection.connect();

        // Выполнение логики для Mexc
        self.behavior.execute();

        println!("Работаем с {}", self.name)
    }
}
