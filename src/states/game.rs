use super::state::Action;
use ggez::event::EventHandler;

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), Action> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), Action> {
        Ok(())
    }
}
