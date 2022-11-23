use {
    super::state::Action,
    crate::{
        game_objects::{game_object::GameObject, player::Player},
        misc::maze::Maze,
    },
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color},
        Context,
    },
};

pub struct Game {
    game_objects: Vec<Box<dyn GameObject>>,
    maze: Maze,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let mut game_objects = Vec::<Box<dyn GameObject>>::new();
        game_objects.push(Box::new(Player::new(ctx)));

        let maze = Maze::new(3, 3);
        maze.print();

        Self { game_objects, maze }
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();

        for game_object in self.game_objects.iter_mut() {
            game_object.update(ctx, dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::BLACK);

        for game_object in self.game_objects.iter_mut() {
            game_object.draw(ctx, &mut canvas);
        }

        canvas.finish(&mut ctx.gfx).unwrap();
        Ok(())
    }
}
