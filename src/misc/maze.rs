use rand::{thread_rng, Rng};
use std::collections::HashSet;

use ggez::mint::Point2;

pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Room>>,
}

impl Maze {
    /// Only pass odd width and height :O
    pub fn new(width: usize, height: usize) -> Self {
        let spawn_pos = Point2 { x: 2, y: 2 };

        let mut grid = Vec::<Vec<Room>>::with_capacity(height);

        for row in 0..height {
            grid.push(Vec::with_capacity(width));
            for _ in 0..width {
                grid[row].push(Room::Empty);
            }
        }

        grid[spawn_pos.y][spawn_pos.x] = Room::Spawn;

        let history = Vec::<Point2<usize>>::new();
        let visited = HashSet::<Point2<usize>>::new();

        Self::recurse(&mut grid, history, visited, spawn_pos, false);
        println!();

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

    // maybe reference instead of move?
    fn recurse(
        grid: &mut Vec<Vec<Room>>,
        mut history: Vec<Point2<usize>>,
        mut visited: HashSet<Point2<usize>>,
        pos: Point2<usize>,
        is_backtracking: bool,
    ) {
        println!();

        if !is_backtracking {
            history.push(pos.clone());
        }

        visited.insert(pos.clone());

        if grid[pos.y][pos.x] == Room::Empty {
            if thread_rng().gen_range(0..5) == 0 {
                grid[pos.y][pos.x] = Room::Null;
            } else {
                grid[pos.y][pos.x] = Room::Base;
            }
        }

        let height = grid.len();
        let width = grid[0].len();
        let mut neighbors = Vec::<Point2<usize>>::new();

        for y in 0..height {
            for x in 0..width {
                if pos != (Point2 { x, y }) {
                    print!("{:?} ", grid[y][x]);
                } else {
                    print!("me_rn ");
                }
            }
            println!();
        }

        let Point2 { x, y } = pos;

        // up
        if !visited.contains(&Point2 { x, y: y - 1 }) && y as i8 - 1 >= 0 {
            neighbors.push(Point2 { x, y: y - 1 });
        }

        // down
        if !visited.contains(&Point2 { x, y: y + 1 }) && y as i8 + 1 <= height as i8 - 1 {
            neighbors.push(Point2 { x, y: y + 1 });
        }

        // left
        if !visited.contains(&Point2 { x: x - 1, y }) && x as i8 - 1 >= 0 {
            neighbors.push(Point2 { x: x - 1, y });
        }

        // right
        if !visited.contains(&Point2 { x: x + 1, y }) && x as i8 + 1 <= width as i8 - 1 {
            neighbors.push(Point2 { x: x + 1, y });
        }

        if visited.len() == width * height || history.len() == 0 {
            return;
        }

        if grid[pos.y][pos.x] == Room::Null || neighbors.len() == 0 {
            drop(history.pop());
            let new_pos = history.last().unwrap().clone();
            return Self::recurse(grid, history, visited, new_pos, true);
        } else {
            let idx = thread_rng().gen_range(0..neighbors.len());
            let neighbor = neighbors.remove(idx);

            return Self::recurse(grid, history, visited, neighbor, false);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Room {
    Spawn,
    Shop,
    Boss,
    Enemy,
    Null,
    Empty,
    Base,
}
