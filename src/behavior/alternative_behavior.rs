use tokio::task::JoinHandle;

use super::behavior_strategy::BehaviorStrategy;

pub struct AlternativeBehavior;

impl BehaviorStrategy for AlternativeBehavior {
    async fn execute(&self) -> JoinHandle<()> {
        tokio::spawn(async {
            println!("Выполняется альтернативная логика");
        })
    }
}
