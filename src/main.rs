use ggez::{event::run, ContextBuilder};
use states::{app::App, game::Game, state::Action};

mod states {
    pub mod app;
    pub mod game;
    pub mod state;
}

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("Dungeon Game", "Theo Lee and Daniel Li")
        .build()
        .unwrap();

    let mut app = App::new();
    app.add_action(Action::Create(Box::new(Game::new())));
    run(ctx, event_loop, app);
}
