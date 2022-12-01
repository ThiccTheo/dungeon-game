use {
    super::state::Action,
    crate::{
        game_objects::{floor::Floor, game_object::create_view, player::Player},
        world::maze::Maze,
    },
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, DrawParam, Image, InstanceArray},
        input::mouse::set_cursor_type,
        mint::Point2,
        winit::window::CursorIcon,
        Context,
    },
    std::collections::HashMap,
};

pub struct Game {
    maze: Maze,
    batches: HashMap<&'static str, InstanceArray>,
    room_indices: Point2<usize>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        set_cursor_type(ctx, CursorIcon::Crosshair);

        let maze = Maze::new(3, 3);
        let room_indices = Point2 { x: 1, y: 1 };

        let mut batches = HashMap::<&'static str, InstanceArray>::new();

        Self::add_batch(ctx, &mut batches, Player::ID);
        Self::add_batch(ctx, &mut batches, Floor::ID);

        Self {
            maze,
            batches,
            room_indices,
        }
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

        let cur_room = &mut self.maze.rooms[self.room_indices.y][self.room_indices.x];

        for i in 0..cur_room.game_objects().unwrap().len() {
            let (this, others) = create_view(cur_room.game_objects().unwrap(), i);
            this.update(ctx, dt, others);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        let cur_room = &mut self.maze.rooms[self.room_indices.y][self.room_indices.x];

        for obj in cur_room.game_objects().unwrap().iter_mut() {
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
