use std::any::Any;

use async_trait::async_trait;

use super::connection_strategy::ConnectionStrategy;

pub struct WebSocketConnection;

#[async_trait]
impl ConnectionStrategy for WebSocketConnection {
    async fn connect(&self, _url: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("WebSocketConnection: No actual connection established");
        // Возвращаем фиктивный успешный результат
        Ok("Placeholder connection".to_string())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl WebSocketConnection {
    pub fn _new() -> Self {
        WebSocketConnection
    }
}
