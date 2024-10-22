use super::connection_strategy::ConnectionStrategy;

pub struct HttpConnection;

impl ConnectionStrategy for HttpConnection {
    fn connect(&self) {
        println!("Подключаемся через Http");
    }
}
