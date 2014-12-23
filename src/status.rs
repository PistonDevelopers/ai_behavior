
/// The result of a behavior or action.
#[deriving(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Show)]
pub enum Status {
    /// The behavior or action succeeded.
    Success,
    /// The behavior or action failed.
    Failure,
    /// The behavior or action is still running.
    Running,
}
