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
    State3,
}

enum Action
{
    DoNothing,
    ChangeState(States),
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
        return Action::ChangeState(States::State3);
    }

    Action::DoNothing
}

fn do_state_3(_state_info: &mut StateInfo) -> Action {
    if random_integer::random_u8(0, 5) == 0 {
        return Action::ChangeState(States::State1);
    }

    Action::DoNothing
}

fn state_transitions(state_info: &mut StateInfo) {
    // State transitions - Code to execute any time you leave a particular state
    match state_info.prev_state {
        States::StartUp => { println!("Finished starting up!") },
        States::State1 => { },
        States::State2 => { },
        States::State3 => { },
    }

    // State transitions - Code to execute on a specific state transition (from/to combination)
    match (state_info.prev_state, state_info.next_state) {
        (_, States::StartUp) => { panic!("WTF!"); },
        (States::State1, States::State2) => { },
        (States::State2, States::State1) => { },
        (States::State3, States::State1) => { println!("Back to the beginning!") },
        (_, _) => { },
    }

    // State transitions - Code to execute any time you enter a particular state
    match state_info.next_state {
        States::StartUp => { },
        States::State1 => { },
        States::State2 => { },
        States::State3 => { },
    }
}

fn main() {
    let initial_state = States::StartUp;
    let mut state_info = StateInfo::new(initial_state);

    loop {
        println!("Processing {:?}", state_info.curr_state);
        let state_func = match state_info.curr_state {
            States::StartUp => do_startup,
            States::State1 => do_state_1,
            States::State2 => do_state_2,
            States::State3 => do_state_3,
        };
        let action = state_func(&mut state_info);

        match action {
            Action::DoNothing => {
                // Do nothing
            },
            Action::ChangeState(next_state) => {
                println!("\nChanging from {:?} to {:?}", state_info.curr_state, next_state);
                state_info.prev_state = state_info.curr_state;
                state_info.next_state = next_state;

                state_transitions(&mut state_info);

                state_info.curr_state = state_info.next_state;
                println!("");
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
