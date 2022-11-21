use std::collections::VecDeque;

use ggez::event::EventHandler;

use super::state::{Action, State};

pub struct App {
    states: Vec<Box<State>>,
    actions: VecDeque<Action>,
}

impl App {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            actions: VecDeque::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push_back(action);
    }

    fn refresh(&mut self) {
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::Create(state) => self.states.push(state),
                Action::Destroy => drop(self.states.pop()),
                Action::Change(state) => {
                    self.states.pop();
                    self.states.push(state);
                }
            }
        }
    }
}

impl EventHandler<()> for App {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ()> {
        self.refresh();
        match self.states.last_mut().unwrap().update(ctx) {
            Ok(()) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ()> {
        match self.states.last_mut().unwrap().draw(ctx) {
            Ok(()) => Ok(()),
            Err(action) => {
                self.add_action(action);
                Ok(())
            }
        }
    }
}
