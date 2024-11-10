use tokio::task::JoinHandle;

use super::{alternative_behavior::AlternativeBehavior, standard_behavior::StandardBehavior};

/*
BehaviorStrategy нельзя использовать как объектный тип (dyn BehaviorStrategy), поскольку он содержит асинхронный метод execute. В Rust асинхронные методы делают тип не-объектным, потому что async функции скрывают тип возвращаемого значения, что нарушает принцип объектного типа. Для решения этой проблемы можно воспользоваться одним из подходов:

Перенести execute в другой трейт, чтобы BehaviorStrategy оставался объектным.
Использовать enum для разных реализаций BehaviorStrategy.
*/

pub trait BehaviorStrategy {
    async fn execute(&self) -> JoinHandle<()>;
}

pub enum Behavior {
    Standard(StandardBehavior),
    Alternative(AlternativeBehavior),
}

impl Behavior {
    pub async fn execute(&self) -> JoinHandle<()> {
        match self {
            Behavior::Standard(behavior) => behavior.execute().await,
            Behavior::Alternative(behavior) => behavior.execute().await,
        }
    }
}
