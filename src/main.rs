use std::{thread, time};

extern crate random_integer;

struct StateInfo {
    prev_state: States,
    curr_state: States,
    next_state: States,
}

impl StateInfo {
    fn new(initial_state: States) -> StateInfo {
        StateInfo {
            prev_state: initial_state,
            curr_state: initial_state,
            next_state: initial_state,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

fn transition_to_startup(_state_info: &mut StateInfo) {
    // Nothing do to here
}

fn transition_to_state_1(_state_info: &mut StateInfo) {
    // TODO: .............................
}

fn transition_to_state_2(_state_info: &mut StateInfo) {
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
    let initial_state = States::StartUp;
    let mut state_info = StateInfo::new(initial_state);

    loop {
        let state_func = match state_info.curr_state {
            States::StartUp => do_startup,
            States::State1 => do_state_1,
            States::State2 => do_state_2,
        };

        match state_func(&mut state_info) {
            Action::DoNothing => {
                // Do nothing
            },
            Action::ChangeState(next_state) => {
                println!("Changing from {:?} to {:?}", state_info.curr_state, next_state);
                state_info.prev_state = state_info.curr_state;
                state_info.next_state = next_state;
                match state_info.next_state {
                    States::StartUp => transition_to_startup(&mut state_info),
                    States::State1 => transition_to_state_1(&mut state_info),
                    States::State2 => transition_to_state_2(&mut state_info),
                }
                state_info.curr_state = state_info.next_state;
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
