use async_trait::async_trait;
use reqwest::{Client, Error, Url};
use std::{any::Any, time::Duration};

use super::connection_strategy::ConnectionStrategy;

pub struct HttpConnection {
    client: Client,
    base_url: Url,
}

impl HttpConnection {
    // Приватный метод для создания клиента
    fn create_client() -> Result<Client, Error> {
        Client::builder()
            .pool_max_idle_per_host(3)
            .timeout(Duration::from_secs(10))
            .build()
    }

    pub async fn new(base_url: &str) -> Result<Self, Error> {
        let client = Self::create_client()?; // Создание клиента через приватный метод

        let base_url = Url::parse(base_url).unwrap();

        // Подготовительный запрос (открывает соединение)
        // client.head(base_url.as_str()).send().await?;

        Ok(HttpConnection { client, base_url })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

#[async_trait]
impl ConnectionStrategy for HttpConnection {
    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn connect(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.client.get(url).send().await?;
        let body = res.text().await?;
        Ok(body)
    }
}

impl Clone for HttpConnection {
    fn clone(&self) -> Self {
        HttpConnection {
            client: self.client.clone(),     // `Client` должен поддерживать `Clone`
            base_url: self.base_url.clone(), // `Url` должен поддерживать `Clone`
        }
    }
}

// Обновляем трейт ConnectionStrategy, чтобы включить метод `as_any`

/* Заглушка - только для метода connection() в фабрике. Далее не используется.
Перебивается нормальным HttpConnection в методе initialize_connection() фабрики */
// pub struct HttpConnectionPlaceholder;

// #[async_trait]
// impl ConnectionStrategy for HttpConnectionPlaceholder {
//     async fn connect(&self, _url: &str) -> Result<String, Box<dyn std::error::Error>> {
//         println!("HttpConnectionPlaceholder: No actual connection established");
//         // Возвращаем фиктивный успешный результат
//         Ok("Placeholder connection".to_string())
//     }
// }
