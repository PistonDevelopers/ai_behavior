#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! AI behavior tree

extern crate piston;
extern crate num;
extern crate rustc_serialize;

pub use behavior::Behavior;
pub use behavior::Behavior::{
    Action,
    After,
    AlwaysSucceed,
    If,
    Fail,
    Pressed,
    Released,
    Select,
    Sequence,
    Wait,
    WaitForever,
    WhenAll,
    WhenAny,
    While,
};
pub use state::State;
pub use status::Status;
pub use status::Status::{
    Failure,
    Success,
    Running,
};

mod behavior;
mod state;
mod status;
