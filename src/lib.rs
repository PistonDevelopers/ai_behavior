#![deny(missing_doc)]

//! AI behavior tree

extern crate event;
extern crate input;
extern crate serialize;

pub use behavior::{
    Action,
    After,
    AlwaysSucceed,
    Behavior,
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
pub use status::{
    Failure,
    Status,
    Success,
    Running,
};

mod behavior;
mod state;
mod status;

