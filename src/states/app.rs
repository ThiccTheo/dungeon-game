use {
    super::state::{Action, State},
    ggez::{event::EventHandler, Context},
    std::collections::VecDeque,
};

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
}

impl EventHandler<()> for App {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ()> {
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

        let curr_state = self.states.last_mut().unwrap();
        match curr_state.update(ctx) {
            Ok(()) => Ok(()),
            Err(_) => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ()> {
        let curr_state = self.states.last_mut().unwrap();
        match curr_state.draw(ctx) {
            Ok(()) => Ok(()),
            Err(action) => {
                self.add_action(action);
                Ok(())
            }
        }
    }
}
