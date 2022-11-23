mod states {
    pub mod app;
    pub mod game;
    pub mod state;
}

mod misc {
    pub mod maze;
}

mod game_objects {
    pub mod game_object;
    pub mod player;
}

use {
    ggez::{
        conf::{WindowMode, WindowSetup},
        event::run,
        ContextBuilder,
    },
    states::{app::App, game::Game, state::Action},
    std::path::PathBuf,
};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut asset_path = root.clone();
    asset_path.push("assets\\imgs");

    let ctx_builder =
        ContextBuilder::new("Dungeon Game", "Theo Lee and Daniel Li").add_resource_path(asset_path);

    let mode = WindowMode::default().dimensions(1280.0, 720.0);
    let setup = WindowSetup::default().title("Dungeon Game");

    let (mut ctx, event_loop) = ctx_builder
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .unwrap();

    let mut app = App::new();
    app.add_action(Action::Create(Box::new(Game::new(&mut ctx))));
    run(ctx, event_loop, app);
}
