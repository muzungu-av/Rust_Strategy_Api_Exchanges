use crate::{
    behavior::behavior_strategy::BehaviorStrategy,
    connection::connection_strategy::ConnectionStrategy,
};

// Структура для биржи Bybit
pub struct Bybit {
    connection: Box<dyn ConnectionStrategy>,
    behavior: Box<dyn BehaviorStrategy>,
}

impl Bybit {
    pub fn new(
        connection: Box<dyn ConnectionStrategy>,
        behavior: Box<dyn BehaviorStrategy>,
    ) -> Self {
        Bybit {
            connection,
            behavior,
        }
    }

    pub fn perform_task(&self) {
        // Подключение к Bybit
        self.connection.connect();

        // Выполнение логики для Bybit
        self.behavior.execute();
    }
}
