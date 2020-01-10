#[derive(PartialEq)]
pub enum CurrentState {
    Running,
    Paused,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Running
    }
}
