mod behavior;
mod connection;
mod exchanges {
    pub mod bybit;
    pub mod exchange_factory;
    pub mod mexc;
}
mod request_state_machine;

use request_state_machine::*;
use state_machine::StateMachine;
use tokio::main;

#[main]
async fn main() {
    // Получаем начальное состояние
    let mut state_machine = StateMachine::new(States::Initial);
    state_machine.add(States::Initial, Box::new(Initial));
    state_machine.add(States::FirstRequest, Box::new(FirstRequest));
    state_machine.add(States::SecondRequest, Box::new(SecondRequest));
    state_machine.add(States::ThirdRequest, Box::new(ThirdRequest));
    state_machine.add(States::Success, Box::new(Success));
    state_machine.add(States::Error, Box::new(Error));

    // Запуск машины состояний
    let _ = state_machine.run();
}
