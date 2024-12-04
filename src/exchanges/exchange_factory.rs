use crate::behavior::alternative_behavior::AlternativeBehavior;
use crate::behavior::behavior_strategy::Behavior;
use crate::behavior::standard_behavior::StandardBehavior;

use crate::connection::connection_strategy::ConnectionStrategy;
use crate::connection::http_connection::HttpConnection;
use crate::connection::web_socket_connection::WebSocketConnection;
// use crate::exchanges::bybit::Bybit;
use crate::exchanges::mexc::Mexc;
use std::sync::Arc;
pub struct ExchangeFactory {
    connection: Option<Box<dyn ConnectionStrategy + Send + Sync>>,
    behavior: Option<Behavior>,
    prepared_url: Option<String>,
}

impl ExchangeFactory {
    pub fn new() -> Self {
        ExchangeFactory {
            connection: None,
            behavior: None,
            prepared_url: None,
        }
    }

    pub async fn connection(mut self, api_type: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match api_type {
            "wss" => {
                self.connection = Some(Box::new(WebSocketConnection));
            }
            "https" => {
                // Установка URL и создание соединения HttpConnection
                self.prepared_url = Some("https://api.mexc.com/api/v3/kyc/status".to_string());
                if let Some(url) = &self.prepared_url {
                    let http_connection = HttpConnection::new(url).await?;
                    self.connection = Some(Box::new(http_connection));
                } else {
                    panic!("URL не установлен");
                }
            }
            _ => panic!("Неизвестный тип подключения API"),
        }
        Ok(self)
    }

    pub fn behavior(mut self, behavior_type: &str) -> Self {
        if let Some(connection) = &self.connection {
            if let Some(http_connection) = connection.as_any().downcast_ref::<HttpConnection>() {
                self.behavior = match behavior_type {
                    "standard" => Some(Behavior::Standard(StandardBehavior::new(Arc::new(
                        http_connection.clone(),
                    )))),
                    "alternative" => Some(Behavior::Alternative(AlternativeBehavior)),
                    _ => panic!("Неизвестный тип поведения"),
                };
            } else {
                panic!("Невозможное приведение типа соединения");
            }
        } else {
            panic!("Соединение не установлено");
        }
        self
    }

    pub fn build_mexc(self) -> Mexc {
        Mexc::new(
            // self.connection.expect("Connection must be set"),
            self.behavior.expect("Behavior must be set"),
        )
    }
}
