mod states {
    pub mod app;
    pub mod game;
    pub mod state;
}

mod misc {
    pub mod math;
}

mod world {
    pub mod maze;
    pub mod room;
    pub mod spawn;
}

mod game_objects {
    pub mod floor;
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
    let mut path = root.clone();
    path.push("assets\\images");

    let ctx_builder =
        ContextBuilder::new("Dungeon Game", "Theo Lee and Daniel Li").add_resource_path(path);

    let mode = WindowMode::default().dimensions(App::WIDTH, App::HEIGHT);
    let setup = WindowSetup::default().title("Dungeon Game");

    let (mut ctx, e_loop) = ctx_builder
        .window_mode(mode)
        .window_setup(setup)
        .build()
        .unwrap();

    let mut app = App::new();
    app.add_action(Action::Create(Box::new(Game::new(&mut ctx))));
    run(ctx, e_loop, app);
}
