use async_trait::async_trait;
use reqwest::Url;

use std::sync::Arc;

// Трейт для обработчика
#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, url: Option<&str>);
}

// Реализация обработчиков для каждого шага
pub struct InitialHandler {
    next: Option<Arc<dyn Handler>>,
    //todo похоже он не используется - нужно бы сюда передать владение
    //todo и регулировать события отключения чтобы снова подключаться
    // client: Arc<reqwest::Client>, // HTTP-клиент
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
    client: Arc<reqwest::Client>,
    url: String,
}

impl FirstRequestHandler {
    pub fn new(
        next: Option<Arc<dyn Handler>>,
        error_handler: Arc<dyn Handler>,
        client: Arc<reqwest::Client>,
        url: String,
    ) -> Self {
        Self {
            next,
            error_handler,
            client,
            url,
        }
    }
}

#[async_trait]
impl Handler for FirstRequestHandler {
    async fn handle(&self, _: Option<&str>) {
        println!("Выполняется ПЕРВЫЙ запрос...");
        match Url::parse(&self.url) {
            Ok(parsed_url) => {
                println!("url == {}", parsed_url);
                let response = self.client.get(parsed_url).send().await;
                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!("Запрос 1 успешен!");
                            if let Some(next) = &self.next {
                                next.handle(Some(&self.url)).await;
                            }
                        } else {
                            println!("Ошибка запроса 1. Статус: {}", res.status());
                            self.error_handler.handle(Some(&self.url)).await;
                        }
                    }
                    Err(err) => {
                        println!("Ошибка выполнения запроса 1: {:?}", err);
                        self.error_handler.handle(Some(&self.url)).await;
                    }
                }
            }
            Err(parse_err) => {
                println!("Ошибка парсинга URL: {:?}", parse_err);
                self.error_handler.handle(None).await;
            }
        }
    }
}

pub struct SecondRequestHandler {
    next: Option<Arc<dyn Handler>>,
    error_handler: Arc<dyn Handler>,
    client: Arc<reqwest::Client>,
    url: String,
}

impl SecondRequestHandler {
    pub fn new(
        next: Option<Arc<dyn Handler>>,
        error_handler: Arc<dyn Handler>,
        client: Arc<reqwest::Client>,
        url: String,
    ) -> Self {
        Self {
            next,
            error_handler,
            client,
            url,
        }
    }
}

#[async_trait]
impl Handler for SecondRequestHandler {
    async fn handle(&self, _: Option<&str>) {
        println!("Выполняется ВТОРОЙ запрос...");
        match Url::parse(&self.url) {
            Ok(parsed_url) => {
                println!("url == {}", parsed_url);
                let response = self.client.get(parsed_url).send().await;
                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!("Запрос 2 успешен!");
                            if let Some(next) = &self.next {
                                next.handle(Some(&self.url)).await;
                            }
                        } else {
                            println!("Ошибка запроса 2. Статус: {}", res.status());
                            self.error_handler.handle(Some(&self.url)).await;
                        }
                    }
                    Err(err) => {
                        println!("Ошибка выполнения запроса 2: {:?}", err);
                        self.error_handler.handle(Some(&self.url)).await;
                    }
                }
            }
            Err(parse_err) => {
                println!("Ошибка парсинга URL: {:?}", parse_err);
                self.error_handler.handle(None).await;
            }
        }
    }
}

pub struct ThirdRequestHandler {
    next: Option<Arc<dyn Handler>>,
    error_handler: Arc<dyn Handler>,
    client: Arc<reqwest::Client>,
    url: String,
}

impl ThirdRequestHandler {
    pub fn new(
        next: Option<Arc<dyn Handler>>,
        error_handler: Arc<dyn Handler>,
        client: Arc<reqwest::Client>,
        url: String,
    ) -> Self {
        Self {
            next,
            error_handler,
            client,
            url,
        }
    }
}

#[async_trait]
impl Handler for ThirdRequestHandler {
    async fn handle(&self, _: Option<&str>) {
        println!("Выполняется ТРЕТИЙ запрос...");
        match Url::parse(&self.url) {
            Ok(parsed_url) => {
                println!("url == {}", parsed_url);
                let response = self.client.get(parsed_url).send().await;
                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!("Запрос 3 успешен!");
                            if let Some(next) = &self.next {
                                next.handle(Some(&self.url)).await;
                            }
                        } else {
                            println!("Ошибка запроса 3. Статус: {}", res.status());
                            self.error_handler.handle(Some(&self.url)).await;
                        }
                    }
                    Err(err) => {
                        println!("Ошибка выполнения запроса 3: {:?}", err);
                        self.error_handler.handle(Some(&self.url)).await;
                    }
                }
            }
            Err(parse_err) => {
                println!("Ошибка парсинга URL: {:?}", parse_err);
                self.error_handler.handle(None).await;
            }
        }
    }
}

pub struct SuccessHandler;

#[async_trait]
impl Handler for SuccessHandler {
    async fn handle(&self, _url: Option<&str>) {
        println!("Все запросы выполнены успешно. Процесс завершен.");
    }
}

pub struct ErrorHandler;

#[async_trait]
impl Handler for ErrorHandler {
    async fn handle(&self, url: Option<&str>) {
        match url {
            Some(url) => println!("*** ErrorHandler - Ошибка запроса по URL: {}", url),
            None => println!("*** ErrorHandler - Процесс завершился с ошибкой."),
        }
    }
}
