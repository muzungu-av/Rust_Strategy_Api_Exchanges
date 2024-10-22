use super::behavior_strategy::BehaviorStrategy;

pub struct StandardBehavior;
impl BehaviorStrategy for StandardBehavior {
    fn execute(&self) {
        println!("Выполняется стандартная логика");
    }
}
