/// The result of a behavior or action.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Status {
    /// The behavior or action succeeded.
    Success,
    /// The behavior or action failed.
    Failure,
    /// The behavior or action is still running.
    Running,
}
