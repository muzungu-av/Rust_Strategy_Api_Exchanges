use crate::behavior::behavior_strategy::Behavior;

pub struct Mexc {
    name: String,
    behavior: Behavior,
}

impl Mexc {
    pub fn new(behavior: Behavior) -> Self {
        let name: String = "MEXC".to_string();
        Mexc { name, behavior }
    }

    pub async fn perform_task(&self) {
        println!("Работаем с {}", self.name);
        // "https://api.mexc.com/"
        if let Err(e) = self.behavior.execute().await.await {
            eprintln!("Ошибка при выполнении задачи: {:?}", e);
        }
    }
}
