use state_machine::{Machine, StateMachine};

// Определяем состояния
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum States {
    Initial,
    FirstRequest,
    SecondRequest,
    ThirdRequest,
    Success,
    Error,
}

// Определяем структуры для каждого состояния
#[derive(Eq, PartialEq, Debug)]
pub struct Initial;
#[derive(Eq, PartialEq, Debug)]
pub struct FirstRequest;
#[derive(Eq, PartialEq, Debug)]
pub struct SecondRequest;
#[derive(Eq, PartialEq, Debug)]
pub struct ThirdRequest;
#[derive(Eq, PartialEq, Debug)]
pub struct Success;
#[derive(Eq, PartialEq, Debug)]
pub struct Error;

// Реализация состояний с переходами
impl Machine<States> for Initial {
    fn update(&self, state: &mut States) -> bool {
        println!("Начальное состояние. Выполнение первого запроса...");
        *state = States::FirstRequest;
        true
    }
}

impl Machine<States> for FirstRequest {
    fn update(&self, state: &mut States) -> bool {
        println!("Первый запрос выполнен. Выполнение второго запроса...");
        *state = States::SecondRequest;
        true
    }
}

impl Machine<States> for SecondRequest {
    fn update(&self, state: &mut States) -> bool {
        println!("Второй запрос выполнен. Выполнение третьего запроса...");
        *state = States::ThirdRequest;
        true
    }
}

impl Machine<States> for ThirdRequest {
    fn update(&self, state: &mut States) -> bool {
        println!("Третий запрос выполнен. Переход к успеху...");
        *state = States::Success;
        true
    }
}

// Реализация для конечных состояний Success и Error
impl Machine<States> for Success {
    fn on_enter(&self, _: &mut States) -> bool {
        println!("Все запросы выполнены успешно. Процесс завершен.");
        true
    }

    fn update(&self, _: &mut States) -> bool {
        // Завершаем процесс и не допускаем дальнейших переходов
        false
    }
}

impl Machine<States> for Error {
    fn on_enter(&self, _: &mut States) -> bool {
        println!("Процесс завершился с ошибкой.");
        true
    }

    fn update(&self, _: &mut States) -> bool {
        // Завершаем процесс и не допускаем дальнейших переходов
        false
    }
}

// Основная функция
// fn main() {
//     let mut state_machine = StateMachine::new(States::Initial);
//     state_machine.add(States::Initial, Box::new(Initial));
//     state_machine.add(States::FirstRequest, Box::new(FirstRequest));
//     state_machine.add(States::SecondRequest, Box::new(SecondRequest));
//     state_machine.add(States::ThirdRequest, Box::new(ThirdRequest));
//     state_machine.add(States::Success, Box::new(Success));
//     state_machine.add(States::Error, Box::new(Error));

//     // Запуск машины состояний
//     let _ = state_machine.run();
// }
