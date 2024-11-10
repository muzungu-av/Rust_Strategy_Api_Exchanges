use crate::behavior::alternative_behavior::AlternativeBehavior;
use crate::behavior::behavior_strategy::{Behavior, BehaviorStrategy};
use crate::behavior::standard_behavior::StandardBehavior;

use crate::connection::connection_strategy::ConnectionStrategy;
use crate::connection::http_connection::HttpConnection;
use crate::connection::web_socket_connection::WebSocketConnection;

use crate::exchanges::bybit::Bybit;
use crate::exchanges::mexc::Mexc;
pub struct ExchangeFactory {
    connection: Option<Box<dyn ConnectionStrategy>>,
    behavior: Option<Behavior>,
}

impl ExchangeFactory {
    pub fn new() -> Self {
        ExchangeFactory {
            connection: None,
            behavior: None,
        }
    }

    // Метод для выбора стратегии подключения на основе переменной окружения
    pub fn connection(mut self, api_type: &str) -> Self {
        self.connection = match api_type {
            "wss" => Some(Box::new(WebSocketConnection)),
            "http" => Some(Box::new(HttpConnection)),
            _ => panic!("Неизвестный тип подключения API"),
        };
        self
    }

    // Метод для выбора стратегии поведения на основе переменной окружения
    pub fn behavior(mut self, behavior_type: &str) -> Self {
        self.behavior = match behavior_type {
            "standard" => Some(Behavior::Standard(StandardBehavior)),
            "alternative" => Some(Behavior::Alternative(AlternativeBehavior)),
            _ => panic!("Неизвестный тип поведения"),
        };
        self
    }

    pub fn build_bybit(self) -> Bybit {
        Bybit::new(
            self.connection.expect("Connection must be set"),
            self.behavior.expect("Behavior must be set"),
        )
    }

    pub fn build_mexc(self) -> Mexc {
        Mexc::new(
            self.connection.expect("Connection must be set"),
            self.behavior.expect("Behavior must be set"),
        )
    }
}
