use crate::{
    behavior::behavior_strategy::BehaviorStrategy,
    connection::connection_strategy::ConnectionStrategy,
};

pub struct Mexc {
    connection: Box<dyn ConnectionStrategy>,
    behavior: Box<dyn BehaviorStrategy>,
}

impl Mexc {
    pub fn new(
        connection: Box<dyn ConnectionStrategy>,
        behavior: Box<dyn BehaviorStrategy>,
    ) -> Self {
        Mexc {
            connection,
            behavior,
        }
    }

    pub fn perform_task(&self) {
        // Подключение к Mexc
        self.connection.connect();

        // Выполнение логики для Mexc
        self.behavior.execute();
    }
}
