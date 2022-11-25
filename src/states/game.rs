use {
    super::state::Action,
    crate::{
        game_objects::{game_object::GameObject, player::Player},
        misc::maze::Maze,
    },
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, Image},
        Context,
    },
    std::collections::HashMap,
};

pub struct Game {
    game_objects: Vec<Box<dyn GameObject>>,
    textures: HashMap<String, Image>,
    maze: Maze,
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let mut game_objects = Vec::<Box<dyn GameObject>>::new();
        game_objects.push(Box::new(Player::new(ctx)));

        let maze = Maze::new(3, 3);
        maze.print();

        let mut textures = HashMap::new();
        textures.insert(
            Player::TEXTURE_ID.to_string(),
            Image::from_path(
                &ctx.gfx,
                format!("\\{}.png", Player::TEXTURE_ID.to_string()),
            )
            .unwrap(),
        );

        Self {
            game_objects,
            textures,
            maze,
        }
    }
}

impl EventHandler<Action> for Game {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let dt = ctx.time.delta().as_secs_f32();

        for obj in self.game_objects.iter_mut() {
            obj.update(ctx, dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, Color::WHITE);

        for obj in self.game_objects.iter_mut() {
            obj.draw(&mut canvas, self.textures.get(&obj.texture_id()).unwrap());
        }

        canvas.finish(&mut ctx.gfx).unwrap();

        Ok(())
    }
}
