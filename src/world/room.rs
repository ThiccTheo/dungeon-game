use {super::spawn::Spawn, crate::game_objects::game_object::GameObject};

pub enum Room {
    Spawn(Spawn),
    Base,
    Null,
    Void,
}

impl Room {
    pub fn game_objects(&mut self) -> Option<&mut Vec<Box<dyn GameObject>>> {
        match self {
            Room::Spawn(spawn) => Some(&mut spawn.game_objects),
            _ => None,
        }
    }
}
