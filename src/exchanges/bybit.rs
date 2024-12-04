// use crate::behavior::behavior_strategy::Behavior;

// pub struct Bybit {
//     name: String,
//     behavior: Behavior,
// }

// impl Bybit {
//     pub fn new(behavior: Behavior) -> Self {
//         let name: String = "BYBIT".to_string();
//         Bybit { name, behavior }
//     }

//     pub async fn perform_task(&self) {
//         println!("Работаем с {}", self.name);
//         // "https://api.mexc.com/"
//         if let Err(e) = self.behavior.execute().await.await {
//             eprintln!("Ошибка при выполнении задачи: {:?}", e);
//         }
//     }
// }
