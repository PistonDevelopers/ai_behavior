use ai_behavior::*;
use input::{Event, UpdateArgs};

#[derive(Clone)]
pub enum MyAction {
    A,
    B,
}

fn main() {
    let behavior = Select(vec![
        Action(MyAction::A),
        Action(MyAction::B),
    ]);
    let mut state: State<MyAction, ()> = State::new(behavior);
    let e: Event = UpdateArgs {dt: 1.0}.into();

    // Prints `A`.
    state.event(&e, &mut |action_args| {
        match action_args.action {
            MyAction::A => println!("A"),
            MyAction::B => println!("B"),
        };
        (Success, action_args.dt)
    });
}
