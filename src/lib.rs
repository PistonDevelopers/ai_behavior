#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! AI behavior tree

extern crate event;
extern crate input;
extern crate rustc_serialize;

pub use behavior::Behavior::{
    self,
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
