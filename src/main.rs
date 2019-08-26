use std::{thread, time};

extern crate random_integer;

struct StateInfo {
    
}

impl StateInfo {
    fn new() -> StateInfo {
        StateInfo { }
    }
}

#[derive(Debug)]
enum States
{
    StartUp,
    State1,
    State2,
}

enum Action
{
    DoNothing,
    ChangeState(States),
}

fn transition_to_startup(_state_info: &mut StateInfo, _prev_state: States) {
    // Nothing do to here
}

fn transition_to_state_1(_state_info: &mut StateInfo, _prev_state: States) {
    // TODO: .............................
}

fn transition_to_state_2(_state_info: &mut StateInfo, _prev_state: States) {
    // TODO: .............................
}

fn do_startup(_state_info: &mut StateInfo) -> Action {
    // Nothing do to here

    // Change state
    Action::ChangeState(States::State1)
}

fn do_state_1(_state_info: &mut StateInfo) -> Action {
    if random_integer::random_u8(0, 5) == 0 {
        return Action::ChangeState(States::State2);
    }

    Action::DoNothing
}

fn do_state_2(_state_info: &mut StateInfo) -> Action {
    if random_integer::random_u8(0, 5) == 0 {
        return Action::ChangeState(States::State1);
    }

    Action::DoNothing
}

fn main() {
    let mut current_state = States::StartUp;
    let mut state_info = StateInfo::new();

    loop {
        let state_func = match current_state {
            States::StartUp => do_startup,
            States::State1 => do_state_1,
            States::State2 => do_state_2,
        };

        match state_func(&mut state_info) {
            Action::DoNothing => {
                // Do nothing
            },
            Action::ChangeState(state) => {
                println!("Changing from {:?} to {:?}", current_state, state);
                match state {
                    States::StartUp => transition_to_startup(&mut state_info, current_state),
                    States::State1 => transition_to_state_1(&mut state_info, current_state),
                    States::State2 => transition_to_state_2(&mut state_info, current_state),
                }
                current_state = state;
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
