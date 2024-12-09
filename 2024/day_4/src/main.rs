struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            data: vec![vec!['.'; width]; height]
        }
    }

    fn from_string(input: &str) -> Grid {
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

    fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    fn set_string_direction(&mut self, x: i32, y: i32, value: &str, direction: Direction) {
        let mut iter = GridIterator::new(self, x, y, direction);
        for c in value.chars() {
            self.set(iter.x.try_into().unwrap(), iter.y.try_into().unwrap(), c);
            if iter.next() == None {
                break;
            }
        }
    }

    fn print(&self) {
        for row in &self.data {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    fn match_east(&self, x: usize, y: usize, pattern: &str) -> bool {
        for (i, c) in pattern.chars().enumerate() {
            if self.get(x + i, y) != c {
                return false;
            }
        }
        true
    }
    
}

use strum_macros::Display;

#[derive(Debug, Display, Clone, Copy)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

struct GridIterator {
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    direction: Direction
}

impl<'a> GridIterator {
    fn new(grid: &Grid, x: i32, y: i32, direction: Direction) -> GridIterator {
        GridIterator {
            width: grid.width,
            height: grid.height,
            x,
            y,
            direction
        }
    }

    fn next(&mut self) -> Option<(i32, i32)> {
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

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    let mut output_grid = Grid::new(input_grid.width, input_grid.height);
    let mut xmas_count = 0;
    let height = input_grid.height;
    let width = input_grid.width;
    for y in 0..height {
        for x in 0..width {
            for direction in vec![
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest
            ] {
                let mut iter = GridIterator::new(&input_grid, x.try_into().unwrap(), y.try_into().unwrap(), direction);
                let mut pattern = String::from("XMAS");
                loop {
                    if input_grid.get(iter.x.try_into().unwrap(), iter.y.try_into().unwrap()) != pattern.chars().next().unwrap() {
                        break
                    }
                    pattern.remove(0);
                    if pattern.len() == 0 {
                        println!("Found XMAS at ({}, {}) in direction {}", y, x, direction.clone());
                        output_grid.set_string_direction(x.try_into().unwrap(), y.try_into().unwrap(), "XMAS", direction);
                        xmas_count += 1;
                        break;
                    }
                    if iter.next().is_none() {
                        break
                    }
                }
            }
        }
        
    }
    println!("XMAS count: {}", xmas_count);
    output_grid.print();
}
