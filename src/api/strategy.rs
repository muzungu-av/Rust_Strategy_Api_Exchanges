// Общий интерфейс (трейд) для стратегий
pub trait ApiStrategy {
    fn execute_task(&self, data: &str);
}

// Стратегия для типа 1 API
pub struct ApiWssStrategy;

impl ApiStrategy for ApiWssStrategy {
    fn execute_task(&self, data: &str) {
        println!("Используется Wss API: Обработка данных - {}", data);
        // Логика для типа 1 API
    }
}

// Стратегия для типа 2 API
pub struct ApiHttpStrategy;

impl ApiStrategy for ApiHttpStrategy {
    fn execute_task(&self, data: &str) {
        println!("Используется Http API: Обработка данных - {}", data);
        // Логика для типа 2 API
    }
}
