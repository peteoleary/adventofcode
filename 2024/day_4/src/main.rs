mod grid;
use grid::{Grid, Direction, GridIterator};

use strum::IntoEnumIterator;

fn part_1() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    let mut output_grid = Grid::new(input_grid.width, input_grid.height);
    let mut xmas_count = 0;
    let height = input_grid.height;
    let width = input_grid.width;
    for y in 0..height {
        for x in 0..width {
            for direction in Direction::iter() {
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

fn part_2() {
    let input = std::fs::read_to_string("src/test_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    let mut output_grid = Grid::new(input_grid.width, input_grid.height);
    let pattern_string = "M.S\n.A.\nM.S\n";
    let mut pattern = Grid::from_string(pattern_string);
    let mut xmas_count = 0;
    for y in 0..input_grid.height {
        for x in 0..input_grid.width {
            for direction in Direction::iter() {
                pattern.set_orientation(direction);
                if input_grid.match_pattern(x, y, &pattern) {
                    println!("Found XMAS at ({}, {})", y, x);
                    xmas_count += 1;
                    break;
                }
            }
        }
        
    }
    println!("XMAS count: {}", xmas_count);
}

fn main() {
    part_2();
}