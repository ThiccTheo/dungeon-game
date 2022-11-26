use {
    super::{room::Room, spawn::Spawn},
    ggez::mint::Point2,
    rand::{thread_rng, Rng},
    std::collections::HashSet,
};

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub rooms: Vec<Vec<Room>>,
}

impl Maze {
    pub fn new(mut w: usize, mut h: usize) -> Self {
        w = w.clamp(3, 5);
        h = h.clamp(3, 5);
        let spawn_pos = Point2 { x: w / 2, y: h / 2 };

        let mut rooms = Vec::<Vec<Room>>::with_capacity(h);

        for row in 0..h {
            rooms.push(Vec::with_capacity(w));
            for _ in 0..w {
                rooms[row].push(Room::Null);
            }
        }

        rooms[spawn_pos.y][spawn_pos.x] = Room::Spawn(Spawn::new(3, 3));

        let history = Vec::<Point2<usize>>::new();
        let visited = HashSet::<Point2<usize>>::new();

        Self::generate(&mut rooms, history, visited, spawn_pos, false);

        Maze {
            width: w,
            height: h,
            rooms,
        }
    }

    fn generate(
        rooms: &mut Vec<Vec<Room>>,
        mut history: Vec<Point2<usize>>,
        mut visited: HashSet<Point2<usize>>,
        pos: Point2<usize>,
        is_backtracking: bool,
    ) {
        if !is_backtracking {
            history.push(pos);
        }
        visited.insert(pos);

        let (w, h) = (rooms.len(), rooms[0].len());
        let Point2 { x, y } = pos;

        if matches!(rooms[y][x], Room::Null) {
            if thread_rng().gen_range(0..3) == 0 {
                rooms[y][x] = Room::Void;
            } else {
                rooms[y][x] = Room::Base;
            }
        }

        let mut neighbors = Vec::<Point2<usize>>::new();
        let mut test_pt;

        test_pt = Point2 { x, y: y - 1 };
        if test_pt.y as i8 >= 0 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x, y: y + 1 };
        if test_pt.y as i8 <= h as i8 - 1 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x: x - 1, y };
        if test_pt.x as i8 >= 0 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x: x + 1, y };
        if test_pt.x as i8 <= w as i8 - 1 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        if visited.len() == w * h {
            return;
        } else if matches!(rooms[y][x], Room::Void) || neighbors.is_empty() {
            history.pop();

            if let Some(new_pos) = history.last().cloned() {
                Self::generate(rooms, history, visited, new_pos, true)
            }
        } else {
            let idx = thread_rng().gen_range(0..neighbors.len());
            let neighbor = neighbors.remove(idx);

            Self::generate(rooms, history, visited, neighbor, false)
        }
    }
}
