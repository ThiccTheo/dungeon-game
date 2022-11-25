use {
    ggez::{event::EventHandler, graphics::Image, Context},
    std::{
        collections::HashMap,
        fmt::{Debug, Formatter, Result as FmtResult},
    },
};

pub type State = dyn EventHandler<Action>;

pub enum Action {
    Create(Box<State>),
    Destroy,
    Change(Box<State>),
}

impl Debug for Action {
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        todo!();
    }
}
