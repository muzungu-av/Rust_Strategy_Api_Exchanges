use crate::behavior::alternative_behavior::AlternativeBehavior;
use crate::behavior::behavior_strategy::BehaviorStrategy;
use crate::behavior::standard_behavior::StandardBehavior;

use crate::connection::connection_strategy::ConnectionStrategy;
use crate::connection::http_connection::HttpConnection;
use crate::connection::web_socket_connection::WebSocketConnection;

use crate::exchanges::bybit::Bybit;
use crate::exchanges::mexc::Mexc;

pub struct ExchangeFactory {
    connection: Option<Box<dyn ConnectionStrategy>>,
    behavior: Option<Box<dyn BehaviorStrategy>>,
}

impl ExchangeFactory {
    pub fn new() -> Self {
        ExchangeFactory {
            connection: None,
            behavior: None,
        }
    }

    pub fn with_websocket_connection(mut self) -> Self {
        self.connection = Some(Box::new(WebSocketConnection));
        self
    }

    pub fn with_http_connection(mut self) -> Self {
        self.connection = Some(Box::new(HttpConnection));
        self
    }

    pub fn with_standard_behavior(mut self) -> Self {
        self.behavior = Some(Box::new(StandardBehavior));
        self
    }

    pub fn with_alternative_behavior(mut self) -> Self {
        self.behavior = Some(Box::new(AlternativeBehavior));
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
