use super::behavior_strategy::BehaviorStrategy;

pub struct AlternativeBehavior;

impl BehaviorStrategy for AlternativeBehavior {
    fn execute(&self) {
        println!("Выполняется альтернативная логика");
    }
}
