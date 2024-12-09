#[derive(Clone, Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<char>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            data: vec![vec!['.'; width]; height]
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

    pub fn set_string_direction(&mut self, x: i32, y: i32, value: &str, direction: Direction) {
        let mut iter = GridIterator::new(self, x, y, direction);
        for c in value.chars() {
            self.set(iter.x.try_into().unwrap(), iter.y.try_into().unwrap(), c);
            if iter.next() == None {
                break;
            }
        }
    }

    pub fn print(&self) {
        for row in &self.data {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn match_pattern(&self, x: usize, y: usize, pattern: &Grid) -> bool {
        for py in 0..pattern.height {
            for px in 0..pattern.width {
                if x + px >= self.width || y + py >= self.height {
                    return false;
                }
                if pattern.get(px, py) != '.' && pattern.get(px, py) != self.get(x + px, y + py) {
                    return false;
                }
            }
        }
        true
    }

    pub fn rotate_right(&self) -> Grid {
        if self.width != self.height {
            panic!("Can't rotate non-square grid");
        }
        let mut new_data = vec![vec!['*'; self.width]; self.height];

        // rotate the top row
        for x in (1..self.width).rev() {
            new_data[0][x] = self.data[0][x - 1];
        }

        // rotate the right column
        for y in (1..self.height).rev() {
            new_data[y][self.width - 1] = self.data[y - 1][self.width - 1];
        }

        // rotate the bottom row
        for x in 0..self.width - 1 {
            new_data[self.height - 1][x] = self.data[self.height - 1][x + 1];
        }

        // rotate the left column
        for y in 0..self.height - 1 {
            new_data[y][0] = self.data[y + 1][0];
        }

        // copy the center, TODO: generalize this
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                new_data[y][x] = self.data[y][x];
            }
        }
        
        Grid {
            width: self.height,
            height: self.width,
            data: new_data
        }
    }

    pub fn set_orientation(&self, direction: Direction) -> Grid {
        let mut new_data = self.data.clone();
        let rotations = match direction {
            Direction::North => {
                0
            },
            Direction::NorthEast => {
                1
            },
            Direction::East => {
                2
            },
            Direction::SouthEast => {
                3
            },
            Direction::South => {
                4
            },
            Direction::SouthWest => {
                5
            },
            Direction::West => {
                6
            },
            Direction::NorthWest => {
                7
            }
        };
        let mut new_grid = self.clone();
        for _ in 0..rotations {
            new_grid = new_grid.rotate_right()
        }
        new_grid.print();
        new_grid
    }
    
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

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

pub struct GridIterator {
    pub x: i32,
    pub y: i32,
    width: usize,
    height: usize,
    direction: Direction
}

impl<'a> GridIterator {
    pub fn new(grid: &Grid, x: i32, y: i32, direction: Direction) -> GridIterator {
        GridIterator {
            width: grid.width,
            height: grid.height,
            x,
            y,
            direction
        }
    }

    pub fn next(&mut self) -> Option<(i32, i32)> {
        match self.direction {
            Direction::North => {
                self.y -= 1;
            },
            Direction::NorthEast => {
                self.y -= 1;
                self.x += 1;
            },
            Direction::East => {
                self.x += 1;  
            },
            Direction::SouthEast => {
                self.y += 1;
                self.x += 1;
            },
            Direction::South => {
                self.y += 1;
            },
            Direction::SouthWest => {
                self.y += 1;
                self.x -= 1;
            },
            Direction::West => {
                self.x -= 1;
            },
            Direction::NorthWest => {
                self.y -= 1;
                self.x -= 1;
            }
        }
       if self.x < self.width.try_into().unwrap() && self.y < self.height.try_into().unwrap() && self.x >= 0 && self.y >= 0 {
           Some((self.x, self.y))
       } else {
           None
       }
    }
}

#[cfg(test)]

#[test]
fn test_grid_rotate() {
    let pattern = Grid::from_string("M.S\n.A.\nM.S\n");
    let test_pattern = Grid::from_string(".M.\nMAS\n.S.\n");
    pattern.print();
    println!("--- pattern: ---");
    pattern.rotate_right().print();
    println!("--- test_pattern: ---");
    test_pattern.print();
    assert_eq!(pattern.rotate_right(), test_pattern);
}