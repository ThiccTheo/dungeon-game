use crate::game_objects::{floor::Floor, game_object::GameObject, player::Player};

pub struct Spawn {
    pub game_objects: Vec<Box<dyn GameObject>>,
    pub width: usize,
    pub height: usize,
}

impl Spawn {
    pub fn new(w: usize, h: usize) -> Self {
        let mut game_objects = Vec::<Box<dyn GameObject>>::new();
        game_objects.push(Box::new(Player::new()));

        for y in 0..h {
            for x in 0..w {
                game_objects.push(Box::new(Floor::new(x, y)));
            }
        }

        Self {
            game_objects,
            width: w,
            height: h,
        }
    }
}
