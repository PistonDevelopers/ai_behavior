#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! AI behavior tree

extern crate input;
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub use behavior::Behavior::{
    self,
    Action,
    After,
    AlwaysSucceed,
    If,
    Fail,
    WaitForPressed,
    WaitForReleased,
    Select,
    Sequence,
    Wait,
    WaitForever,
    WhenAll,
    WhenAny,
    While,
};
pub use state::{ ActionArgs, State, RUNNING };
pub use status::Status::{
    self,
    Failure,
    Success,
    Running,
};

mod behavior;
mod state;
mod status;
