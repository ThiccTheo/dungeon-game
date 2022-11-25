use ggez::graphics::{InstanceArray, DrawParam};

use {
    super::state::Action,
    crate::{
        game_objects::{floor::Floor, game_object::GameObject, player::Player},
        world::maze::Maze,
    },
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, Image},
        input::mouse::set_cursor_type,
        winit::window::CursorIcon,
        Context,
    },
    std::collections::HashMap,
};

pub struct Game {
    maze: Maze,
    batches: HashMap<String, InstanceArray>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        set_cursor_type(ctx, CursorIcon::Crosshair);

        let mut game_objects = Vec::<Box<dyn GameObject>>::new();
        game_objects.push(Box::new(Player::new()));

        let maze = Maze::new(3, 3);

        let mut batches = HashMap::<String, InstanceArray>::new();

        Self::add_batch(ctx, &mut batches, Player::TEXTURE_ID);
        Self::add_batch(ctx, &mut batches, Floor::TEXTURE_ID);

        Self { maze, batches }
    }

    pub fn add_batch(ctx: &Context, batches: &mut HashMap<String, InstanceArray>, tex_id: &str) {
        let batch = InstanceArray::new(
            &ctx.gfx,
            Image::from_path(&ctx.gfx, format!("\\{}.png", tex_id.to_string())).unwrap(),
        );
        batches.insert(tex_id.to_string(), batch);
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();

        for obj in self.maze.rooms[1][1].game_objects().unwrap().iter_mut() {
            obj.update(ctx, dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        for obj in self.maze.rooms[1][1].game_objects().unwrap().iter_mut() {
            obj.draw(&mut canvas, self.batches.get_mut(&obj.texture_id()).unwrap());
        }

        for (.., batch) in self.batches.iter_mut() {
            canvas.draw(batch, DrawParam::default());
            batch.clear();
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        Ok(())
    }
}
