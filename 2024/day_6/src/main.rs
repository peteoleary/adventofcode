
mod grid;
use grid::{Grid, Direction, GridIterator};

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let mut input_grid = Grid::from_string(&input);
    input_grid.print();
    let mut grid_iterator = GridIterator::new(&input_grid, input_grid.current_x, input_grid.current_y, Direction::South);
    grid_iterator.set_direction_from_char(input_grid.get(input_grid.current_x, input_grid.current_y));
    let mut position_count = 1;
    loop {
        if input_grid.get(grid_iterator.x, grid_iterator.y) == '.' {
            position_count += 1;
        }
        input_grid.set(grid_iterator.x, grid_iterator.y, 'X');
        match grid_iterator.next() {
            Some((x, y)) => {
                println!("({}, {}): {}", x, y, input_grid.get(x, y));
                if input_grid.get(x, y) == '#' {
                    grid_iterator.turn_right();
                    grid_iterator.turn_right()
                } else {
                    grid_iterator.x = x;
                    grid_iterator.y = y;
                }
            },
            None => break
        }
    }
    input_grid.print();
    println!("Position count: {}", position_count);
}
