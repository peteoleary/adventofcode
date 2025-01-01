mod grid;
use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
struct Antenna {
    positions: (Point, Point),
    antinodes: (Option<Point>, Option<Point>)
}

impl PartialEq for Antenna {
    fn eq(&self, other: &Self) -> bool {
        self.positions == other.positions    
    }
}

impl Antenna {
    fn new(a: Point, b:Point) -> Antenna {
        let mut positions = vec![a, b];
        positions.sort();
        Antenna{ positions: (positions[0], positions[1]), antinodes: (None, None) }
    }

    fn point_to_antinode(&self, point: Point, grid: &Grid) -> Option<Point> {
        if point.x < 0 || point.y < 0 || point.x >= grid.width as i32 || point.y >= grid.height as i32 {
            None
        } else {
            Some(point)
        }
    }

    fn calc_antinodes(&mut self, grid: &Grid) -> &mut Self {
        let diff = self.diff();
        let point_1 = Point::new(self.positions.0.x - diff.0, self.positions.0.y - diff.1);
        let point_2 = Point::new(self.positions.1.x + diff.0, self.positions.1.y + diff.1);
        self.antinodes = (self.point_to_antinode(point_1, grid), self.point_to_antinode(point_2, grid));
        self
    }

    fn diff(&self) -> (i32, i32) {
        let x = self.positions.1.x as i32 - self.positions.0.x as i32;
        let y = self.positions.1.y as i32 - self.positions.0.y as i32;
        (x, y)
    }
}

fn find_all_points(grid: &Grid, c: char) -> Vec<Point> {
    let mut points = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == c {
                points.push(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    points
}

fn connect_all_points(points: Vec<Point>, grid: &Grid ) -> Vec<Antenna> {
    let mut connections = Vec::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            let mut new_antenna = Antenna::new(points[i], points[j]);
            if i != j && !connections.contains(&new_antenna) {
                new_antenna.calc_antinodes(grid);
                connections.push(new_antenna);
            }
        }
    }
    connections
}

fn set_grid_antinode(grid: &mut Grid, antinode: Option<Point>) -> bool {
    if let Some(a) = antinode {
        let (x, y) = (a.x.try_into().unwrap(), a.y.try_into().unwrap());
        if grid.get(x, y) != '#' {
            grid.set(x, y, '#');
            return true;
        }
    }
    false
}

fn handle_one_layer(grid: &Grid, c: char) -> (Grid, i32) {
    let layer = find_all_points(&grid, c);
    // let layer_0 = find_all_points(&input_grid, '0');
    let connections = connect_all_points(layer, grid);
    let mut new_grid = grid.clone();
    let mut count = 0;
    connections.iter().for_each(|cn: &Antenna| {
        if set_grid_antinode(&mut new_grid, cn.antinodes.0) {
            count += 1;
        }
        if set_grid_antinode(&mut new_grid, cn.antinodes.1) {
            count += 1;
        }
    });
    (new_grid, count)
}

fn find_layers(input: &str) -> Vec<char> {
    let mut layers = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            if c != '.' && !layers.contains(&c) {
                layers.push(c);
            }
        }
    }
    layers
}

fn merge_grids(grids: Vec<Grid>) -> (Grid, i32) {
    let mut count = 0;
    let mut new_grid = Grid::new(grids[0].width, grids[0].height);
    for i in 0..grids.len() {
        for y in 0..grids[i].height {
            for x in 0..grids[i].width {
                if grids[i].get(x, y) == '#' && new_grid.get(x, y) != '#' {
                    new_grid.set(x, y, '#');
                    count += 1;
                }
            }
        }
    }
    (new_grid, count)
}

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let input_grid = Grid::from_string(&input);
    let layers = find_layers(&input);
    println!("input_grid");
    input_grid.print();
    let mut output_grids = Vec::new();
    for layer in layers {
        let (layer_grid, layer_count) = handle_one_layer(&input_grid, layer);
        println!("grid after layer: {}", layer);
        layer_grid.print();
        output_grids.push(layer_grid);
    }
    let (result_grid, count) = merge_grids(output_grids);
    println!("result_grid");
    result_grid.print();
    println!("count: {}", count);
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
        let grid = Grid::from_string("A.B\n0.1");
        let connections = connect_all_points(points, &grid);
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0], Antenna::new(Point::new(0, 0), Point::new(1, 1)));
    }
}