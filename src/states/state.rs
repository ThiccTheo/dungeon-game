use ggez::event::EventHandler;
use std::fmt::{Debug, Result as FmtResult};

pub type State = dyn EventHandler<Action>;

pub enum Action {
    Create(Box<State>),
    Destroy,
    Change(Box<State>),
}

impl Debug for Action {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> FmtResult {
        todo!();
    }
}
