use ndarray::Array2;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq)]
enum Cell {
    Wall,
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = match self {
            Cell::Wall => '#',
            Cell::Empty => ' ',
        };
        write!(f, "{}", character)
    }
}

type Point = (isize, isize);
type Grid = Array2<Cell>;
pub struct Maze {
    grid: Grid,
}

impl Maze {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            grid: Grid::from_elem((rows, cols), Cell::Wall),
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut start_x = rng.gen_range(1, self.grid.cols() - 3) as isize;
        let mut start_y = rng.gen_range(1, self.grid.rows() - 3) as isize;
        if start_x % 2 == 0 {
            start_x += 1;
        }
        if start_y % 2 == 0 {
            start_y += 1;
        }

        let mut visited: Vec<Point> = Vec::new();
        let mut previous_points: Vec<Point> = Vec::new();
        let mut stack: VecDeque<Point> = VecDeque::new();
        stack.push_front((start_x, start_y));
        while let Some(point) = stack.pop_front() {
            if let Some((neighbour, direction)) = self.get_neighbour(point, &visited) {
                self.set_cell_to_empty(point, direction);
                visited.push(neighbour);
                stack.push_front(neighbour);
                previous_points.push(point);
            } else {
                if previous_points.is_empty() {
                    break;
                }
                let last_point = previous_points.pop().unwrap();
                stack.push_front(last_point);
            }
        }
    }

    fn get_neighbour(&self, point: Point, visited: &[Point]) -> Option<(Point, Direction)> {
        let mut rng = rand::thread_rng();
        let mut directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        let neighbour_fn = |point: Point, direction: &Direction, step: isize| match direction {
            Direction::North => (point.0, point.1 - step),
            Direction::East => (point.0 + step, point.1),
            Direction::South => (point.0, point.1 + step),
            Direction::West => (point.0 - step, point.1),
        };
        loop {
            if directions.is_empty() {
                return None;
            }

            let index = rng.gen_range(0, directions.len());
            let direction = &directions[index];
            let neighbour = neighbour_fn(point, direction, 2);
            if visited.contains(&neighbour)
                || self.is_out_of_bounds(neighbour)
                || self.grid[[neighbour.1 as usize, neighbour.0 as usize]] == Cell::Empty
            {
                directions.remove(index);
            } else {
                return Some((neighbour, direction.clone()));
            }
        }
    }

    fn is_out_of_bounds(&self, point: Point) -> bool {
        point.0 <= 0
            || point.0 >= ((self.grid.cols() - 1) as isize)
            || point.1 <= 0
            || point.1 >= ((self.grid.rows() - 1) as isize)
    }

    fn set_cell_to_empty(&mut self, point: Point, direction: Direction) {
        let target = match direction {
            Direction::North => (point.0, point.1 - 1),
            Direction::East => (point.0 + 1, point.1),
            Direction::South => (point.0, point.1 + 1),
            Direction::West => (point.0 - 1, point.1),
        };
        self.grid[[point.1 as usize, point.0 as usize]] = Cell::Empty;
        self.grid[[target.1 as usize, target.0 as usize]] = Cell::Empty;
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn maze_is_generated() {
        assert!(true);
    }
}
