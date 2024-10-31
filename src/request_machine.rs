use async_trait::async_trait;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Трейт для обработчика
#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self);
}

// Реализация обработчиков для каждого шага
pub struct InitialHandler {
    next: Option<Arc<dyn Handler>>,
}

impl InitialHandler {
    pub fn new(next: Option<Arc<dyn Handler>>) -> Self {
        Self { next }
    }
}

#[async_trait]
impl Handler for InitialHandler {
    async fn handle(&self) {
        println!("Начальное состояние...");
        if let Some(next) = &self.next {
            next.handle().await;
        }
    }
}

pub struct FirstRequestHandler {
    next: Option<Arc<dyn Handler>>,
    error_handler: Arc<dyn Handler>,
}

impl FirstRequestHandler {
    pub fn new(next: Option<Arc<dyn Handler>>, error_handler: Arc<dyn Handler>) -> Self {
        Self {
            next,
            error_handler,
        }
    }
}

#[async_trait]
impl Handler for FirstRequestHandler {
    async fn handle(&self) {
        println!("Выполняется первый запрос...");

        let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1").await;
        if let Ok(res) = response {
            if res.status().is_success() {
                println!("Запрос 1 успешен!");
                if let Some(next) = &self.next {
                    next.handle().await;
                }
            } else {
                println!("Ошибка запроса 1");
                self.error_handler.handle().await
            }
        } else {
            println!("Ошибка запроса 1");
            self.error_handler.handle().await
        }
    }
}

pub struct SecondRequestHandler {
    next: Option<Arc<dyn Handler>>,
    error_handler: Arc<dyn Handler>,
}

impl SecondRequestHandler {
    pub fn new(next: Option<Arc<dyn Handler>>, error_handler: Arc<dyn Handler>) -> Self {
        Self {
            next,
            error_handler,
        }
    }
}

#[async_trait]
impl Handler for SecondRequestHandler {
    async fn handle(&self) {
        println!("Выполняется второй запрос...");

        let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/2").await;
        if let Ok(res) = response {
            if res.status().is_success() {
                println!("Запрос 2 успешен!");
                if let Some(next) = &self.next {
                    next.handle().await;
                }
            } else {
                println!("Ошибка запроса 2. Переход в ErrorHandler.");
                self.error_handler.handle().await
            }
        } else {
            println!("Ошибка запроса 2. Переход в ErrorHandler.");
            self.error_handler.handle().await
        }
    }
}

pub struct ThirdRequestHandler {
    next: Option<Arc<dyn Handler>>,
    error_handler: Arc<dyn Handler>,
}

impl ThirdRequestHandler {
    pub fn new(next: Option<Arc<dyn Handler>>, error_handler: Arc<dyn Handler>) -> Self {
        Self {
            next,
            error_handler,
        }
    }
}

#[async_trait]
impl Handler for ThirdRequestHandler {
    async fn handle(&self) {
        println!("Выполняется Третий запрос...");

        let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/3").await;
        if let Ok(res) = response {
            if res.status().is_success() {
                println!("Запрос 3 успешен!");
                if let Some(next) = &self.next {
                    next.handle().await;
                }
            } else {
                println!("Ошибка запроса 3");
                self.error_handler.handle().await
            }
        } else {
            println!("Ошибка запроса 3");
            self.error_handler.handle().await
        }
    }
}

pub struct SuccessHandler;

#[async_trait]
impl Handler for SuccessHandler {
    async fn handle(&self) {
        println!("Все запросы выполнены успешно. Процесс завершен.");
    }
}

pub struct ErrorHandler;

#[async_trait]
impl Handler for ErrorHandler {
    async fn handle(&self) {
        println!("ErrorHandler - Процесс завершился с ошибкой.");
    }
}
