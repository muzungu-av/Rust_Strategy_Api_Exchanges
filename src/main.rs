use dotenv::dotenv;
use std::env;

// Общий интерфейс (трейд) для стратегий
trait ApiStrategy {
    fn execute_task(&self, data: &str);
}

// Стратегия для типа 1 API
struct ApiWssStrategy;

impl ApiStrategy for ApiWssStrategy {
    fn execute_task(&self, data: &str) {
        println!("Используется Wss API: Обработка данных - {}", data);
        // Логика для типа 1 API
    }
}

// Стратегия для типа 2 API
struct ApiHttpStrategy;

impl ApiStrategy for ApiHttpStrategy {
    fn execute_task(&self, data: &str) {
        println!("Используется Http API: Обработка данных - {}", data);
        // Логика для типа 2 API
    }
}

// Сервис, который использует стратегию
struct ApiService {
    strategy: Box<dyn ApiStrategy>,
}

impl ApiService {
    fn new(strategy: Box<dyn ApiStrategy>) -> Self {
        ApiService { strategy }
    }

    fn perform_task(&self, data: &str) {
        self.strategy.execute_task(data);
    }
}

fn main() {
    dotenv().ok();
    let api_type = env::var("API_TYPE").unwrap_or_else(|_| "unknown".to_string());
    println!("{}", api_type);
    // Определяем стратегию на основе переменной окружения
    let strategy: Box<dyn ApiStrategy> = match api_type.as_str() {
        "wss" => Box::new(ApiWssStrategy),
        "http" => Box::new(ApiHttpStrategy),
        _ => panic!("Неизвестный тип API"),
    };

    // Создаем сервис с выбранной стратегией
    let api_service = ApiService::new(strategy);

    api_service.perform_task("Пример данных");
}
