
mod grid;
use grid::{Grid, Direction, GridIterator};

fn try_one_loop(grid: &Grid, x: usize, y: usize) -> bool {
    let mut input_grid = grid.clone();
    if input_grid.current_x == x && input_grid.current_y == y {
        return true;
    }
    input_grid.set(x, y, 'O');
    let mut grid_iterator = GridIterator::new(&input_grid, input_grid.current_x, input_grid.current_y, Direction::South);
    grid_iterator.set_direction_from_char(input_grid.get(input_grid.current_x, input_grid.current_y));
    input_grid.set(input_grid.current_x, input_grid.current_y, '.');
    let mut position_count = 1;
    loop {
        match input_grid.get(grid_iterator.x, grid_iterator.y) {
            '.' => {
                position_count += 1;
            },
            '^' => {
                if grid_iterator.get_direction() == Direction::North {
                    input_grid.print();
                    return false;
                }
            },
            '>' => {
                if grid_iterator.get_direction() == Direction::East  {
                    input_grid.print();
                    return false;
                }
            },
            'v' => {
                if grid_iterator.get_direction() == Direction::South {
                    input_grid.print();
                    return false;
                }
            },
            '<' => {
                if grid_iterator.get_direction() == Direction::West {
                    input_grid.print();
                    return false;
                }
            },
            _ => ()
        }
        match grid_iterator.next() {
            Some((x, y)) => {
                match input_grid.get(x, y) {
                    '#' => {
                        grid_iterator.turn_right();
                        grid_iterator.turn_right();
                    },
                    'O'  => {
                        grid_iterator.turn_right();
                        grid_iterator.turn_right();
                    },
                    _ => {
                        input_grid.set(grid_iterator.x, grid_iterator.y, grid_iterator.get_trail_char());
                        grid_iterator.x = x;
                        grid_iterator.y = y;
                    }
                }
            },
            None => break
        }
    }
    true
}

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    let mut count = 0;
    for x in 0..input_grid.width {
        for y in 0..input_grid.height {
            if !try_one_loop(&input_grid, x, y) {
                println!("False at {}, {}", x, y);
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
}
