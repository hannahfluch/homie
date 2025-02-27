use std::ops::Not;

/// State that buddy can be in at any time.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub(crate) enum State {
    #[default]
    Idle,
    Running,
    Click,
}

impl Not for State {
    type Output = State;

    fn not(self) -> Self::Output {
        match self {
            State::Running | State::Click => State::Idle,
            State::Idle => State::Running,
        }
    }
}
