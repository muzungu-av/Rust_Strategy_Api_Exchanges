use super::connection_strategy::ConnectionStrategy;

pub struct WebSocketConnection;

impl ConnectionStrategy for WebSocketConnection {
    fn connect(&self) {
        println!("Подключаемся через WebSocket");
    }
}
