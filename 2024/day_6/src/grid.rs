use std::collections::HashMap;

use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, Clone, Copy, EnumIter)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

use lazy_static::lazy_static;

lazy_static! {
    static ref DIRECTIONS: HashMap<char, Direction> = {
        let mut map = HashMap::new();
        map.insert('^', Direction::North);
        map.insert('>', Direction::East);
        map.insert('V', Direction::South);
        map.insert('<', Direction::West);
        map
    };
}

pub struct GridIterator {
    pub x: usize,
    pub y: usize,
    width: usize,
    height: usize,
    direction: Direction
}

impl<'a> GridIterator {
    pub fn new(grid: &Grid, x: usize, y: usize, direction: Direction) -> GridIterator {
        GridIterator {
            width: grid.width,
            height: grid.height,
            x,
            y,
            direction
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_direction_from_char(&mut self, direction_char: char) {
        self.direction = *DIRECTIONS.get(&direction_char).unwrap();
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North
        }
    }

    pub fn next(&mut self) -> Option<(usize, usize)> {
        let mut x: i32 = self.x.try_into().unwrap();
        let mut y: i32 = self.y.try_into().unwrap();
        match self.direction {
            Direction::North => {
                y -= 1;
            },
            Direction::NorthEast => {
                y -= 1;
                x += 1;
            },
            Direction::East => {
                x += 1;  
            },
            Direction::SouthEast => {
                y += 1;
                x += 1;
            },
            Direction::South => {
                y += 1;
            },
            Direction::SouthWest => {
                y += 1;
                x -= 1;
            },
            Direction::West => {
                x -= 1;
            },
            Direction::NorthWest => {
                y -= 1;
                x -= 1;
            }
        }
       if x < self.width.try_into().unwrap() && y < self.height.try_into().unwrap() && x >= 0 && y >= 0 {
           Some((x.try_into().unwrap(), y.try_into().unwrap()))
       } else {
           None
       }
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub current_x: usize,
    pub current_y: usize,
    data: Vec<Vec<char>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            data: vec![vec!['.'; width]; height],
            current_x: 0,
            current_y: 0
        }
    }

    pub fn from_string(input: &str) -> Grid {
        let mut lines = input.lines();
        let width = lines.next().unwrap().len();
        let height = input.lines().count();
        let mut grid = Grid::new(width, height);
        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                grid.set(x, y, cell);
                if DIRECTIONS.contains_key(&cell) {
                    grid.current_x = x;
                    grid.current_y = y;
                }
            }
        }
        grid
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    pub fn print(&self) {
        for row in &self.data {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]

#[test]
fn test_grid() {
    let pattern = Grid::from_string("...\n.^.\n...\n");
    pattern.print();
    assert!(pattern.current_x == 1 && pattern.current_y == 1);
}