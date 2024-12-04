use std::any::Any;

use async_trait::async_trait;

#[async_trait]

pub trait ConnectionStrategy: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    async fn connect(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
}
