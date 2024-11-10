use async_trait::async_trait;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// Трейт для обработчика
#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, url: Option<&str>);
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
    async fn handle(&self, _: Option<&str>) {
        println!("Начальное состояние...");
        if let Some(next) = &self.next {
            next.handle(None).await;
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
    async fn handle(&self, _: Option<&str>) {
        println!("Выполняется первый запрос...");

        let url = "https://jsonplaceholder.typicode.com/posts/1";
        // Использование surf для выполнения GET-запроса
        let response = surf::get(url).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Запрос 1 успешен!");
                    if let Some(next) = &self.next {
                        next.handle(Some(url)).await;
                    }
                } else {
                    println!("Ошибка запроса 1. Статус: {}", res.status());
                    self.error_handler.handle(Some(url)).await;
                }
            }
            Err(err) => {
                println!("Ошибка выполнения запроса 1: {:?}", err);
                self.error_handler.handle(Some(url)).await;
            }
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
    async fn handle(&self, _: Option<&str>) {
        println!("Выполняется второй запрос...");

        // Использование surf для выполнения GET-запроса
        let url = "https://jsonplaceholder.typicode.com/posts/2";
        let response = surf::get(url).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Запрос 2 успешен!");
                    if let Some(next) = &self.next {
                        next.handle(Some(url)).await;
                    }
                } else {
                    println!("Ошибка запроса 2. Статус: {}", res.status());
                    self.error_handler.handle(Some(url)).await;
                }
            }
            Err(err) => {
                println!("Ошибка выполнения запроса 2: {:?}", err);
                self.error_handler.handle(Some(url)).await;
            }
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
    async fn handle(&self, url: Option<&str>) {
        println!("Выполняется Третий запрос...");

        let url = "https://jsonplaceholder.typicode.com/posts/3";
        let response = surf::get(url).await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    println!("Запрос 3 успешен!");
                    if let Some(next) = &self.next {
                        next.handle(Some(url)).await;
                    }
                } else {
                    println!("Ошибка запроса 3. Статус: {}", res.status());
                    self.error_handler.handle(Some(url)).await;
                }
            }
            Err(err) => {
                println!("Ошибка выполнения запроса 3: {:?}", err);
                self.error_handler.handle(Some(url)).await;
            }
        }
    }
}

pub struct SuccessHandler;

#[async_trait]
impl Handler for SuccessHandler {
    async fn handle(&self, url: Option<&str>) {
        println!("Все запросы выполнены успешно. Процесс завершен.");
    }
}

pub struct ErrorHandler;

#[async_trait]
impl Handler for ErrorHandler {
    async fn handle(&self, url: Option<&str>) {
        match url {
            Some(url) => println!("ErrorHandler - Ошибка запроса по URL: {}", url),
            None => println!("ErrorHandler - Процесс завершился с ошибкой."),
        }
    }
}
