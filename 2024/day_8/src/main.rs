mod grid;
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Antenna {
    positions: (Point, Point),
    antinodes: Option<(Point, Point)>
}

impl Antenna {
    fn new(a: Point, b:Point) -> Antenna {
        let mut positions = vec![a, b];
        positions.sort();
        Antenna{ positions: (positions[0], positions[1]), antinodes: None }
    }
}

fn find_all_points(grid: &Grid, c: char) -> Vec<Point> {
    let mut points = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == c {
                points.push(Point::new(x, y));
            }
        }
    }
    points
}

fn connect_all_points(points: Vec<Point>) -> Vec<Antenna> {
    let mut connections = Vec::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            let new_antenna = Antenna::new(points[i], points[j]);
            if i != j && !connections.contains(&new_antenna) {
                connections.push(new_antenna);
            }
        }
    }
    connections
}

fn handle_one_layer(grid: &Grid, c: char) -> Vec<Antenna> {
    let layer = find_all_points(&grid, c);
    // let layer_0 = find_all_points(&input_grid, '0');
    let connections = connect_all_points(layer);
    println!("Layer: {:?}", c);
    connections.iter().for_each(|cn| {
        println!("{:?}", cn);
    });
    connections
}

fn main() {
    let input = std::fs::read_to_string("src/test_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    handle_one_layer(&input_grid, 'A');
    handle_one_layer(&input_grid, '0');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_points() {
        let input = "A.B\n0.1";
        let grid = Grid::from_string(input);
        let points = find_all_points(&grid, 'A');
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], Point::new(0, 0));
    }

    #[test]
    fn test_connect_all_points() {
        let points = vec![Point::new(0, 0), Point::new(1, 1)];
        let connections = connect_all_points(points);
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0], Antenna::new(Point::new(0, 0), Point::new(1, 1)));
    }
}