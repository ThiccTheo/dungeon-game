use {
    ggez::mint::Point2,
    rand::{thread_rng, Rng},
    std::collections::HashSet,
};

#[derive(Debug, PartialEq)]
pub enum Room {
    Spawn,
    Base,
    Null,
    Void,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Room>>,
}

impl Maze {
    pub fn new(mut width: usize, mut height: usize) -> Self {
        width = width.clamp(3, 5);
        height = height.clamp(3, 5);
        let spawn_pos = Point2 {
            x: width / 2,
            y: height / 2,
        };

        let mut grid = Vec::<Vec<Room>>::with_capacity(height);

        for row in 0..height {
            grid.push(Vec::with_capacity(width));
            for _ in 0..width {
                grid[row].push(Room::Null);
            }
        }

        grid[spawn_pos.y][spawn_pos.x] = Room::Spawn;

        let history = Vec::<Point2<usize>>::new();
        let visited = HashSet::<Point2<usize>>::new();

        Self::recurse(&mut grid, history, visited, spawn_pos, false);

        Maze {
            width,
            height,
            grid,
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:?} ", self.grid[y][x]);
            }
            println!();
        }
    }

    fn recurse(
        grid: &mut Vec<Vec<Room>>,
        mut history: Vec<Point2<usize>>,
        mut visited: HashSet<Point2<usize>>,
        pos: Point2<usize>,
        is_backtracking: bool,
    ) {
        if !is_backtracking {
            history.push(pos);
        }
        visited.insert(pos);

        let height = grid.len();
        let width = grid[0].len();
        let Point2 { x, y } = pos;

        if grid[y][x] == Room::Null {
            if thread_rng().gen_range(0..3) == 0 {
                grid[y][x] = Room::Void;
            } else {
                grid[y][x] = Room::Base;
            }
        }

        let mut neighbors = Vec::<Point2<usize>>::new();
        let mut test_pt;

        test_pt = Point2 { x, y: y - 1 };
        if test_pt.y as i8 >= 0 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x, y: y + 1 };
        if test_pt.y as i8 <= height as i8 - 1 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x: x - 1, y };
        if test_pt.x as i8 >= 0 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        test_pt = Point2 { x: x + 1, y };
        if test_pt.x as i8 <= width as i8 - 1 && !visited.contains(&test_pt) {
            neighbors.push(test_pt);
        }

        if visited.len() == width * height {
            return;
        } else if grid[y][x] == Room::Void || neighbors.is_empty() {
            history.pop();

            if let Some(new_pos) = history.last().cloned() {
                Self::recurse(grid, history, visited, new_pos, true)
            }
        } else {
            let idx = thread_rng().gen_range(0..neighbors.len());
            let neighbor = neighbors.remove(idx);

            Self::recurse(grid, history, visited, neighbor, false)
        }
    }
}
