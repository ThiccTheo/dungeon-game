use {
    super::state::Action,
    crate::{
        game_objects::{floor::Floor, player::Player},
        world::maze::Maze,
    },
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, DrawParam, Image, InstanceArray},
        input::mouse::set_cursor_type,
        winit::window::CursorIcon,
        Context,
    },
    std::collections::HashMap,
};

pub struct Game {
    maze: Maze,
    batches: HashMap<&'static str, InstanceArray>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        set_cursor_type(ctx, CursorIcon::Crosshair);

        let maze = Maze::new(3, 3);

        let mut batches = HashMap::<&'static str, InstanceArray>::new();

        Self::add_batch(ctx, &mut batches, Player::ID);
        Self::add_batch(ctx, &mut batches, Floor::ID);

        Self { maze, batches }
    }

    pub fn add_batch(
        ctx: &Context,
        batches: &mut HashMap<&'static str, InstanceArray>,
        id: &'static str,
    ) {
        let batch = InstanceArray::new(
            &ctx.gfx,
            Image::from_path(&ctx.gfx, format!("\\{id}.png")).unwrap(),
        );
        batches.insert(id, batch);
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();

        for i in 0..self.maze.rooms[1][1].game_objects().unwrap().len() {
            let (before, tmp) = self.maze.rooms[1][1]
                .game_objects()
                .unwrap()
                .split_at_mut(i);
            let (this, after) = tmp.split_first_mut().unwrap();
            let others = before.iter_mut().chain(after.iter_mut());
            this.update(ctx, dt, others);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        for obj in self.maze.rooms[1][1].game_objects().unwrap().iter_mut() {
            obj.draw(ctx, &mut canvas, self.batches.get_mut(&obj.id()).unwrap());
        }

        for batch in self.batches.values_mut() {
            canvas.draw(batch, DrawParam::default());
            batch.clear();
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        Ok(())
    }
}
