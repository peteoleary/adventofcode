
#[derive(Clone)]
struct Diskfile {
    id: usize,
    positions: Vec<usize>
}

struct Diskmap {
    files: Vec<Diskfile>,
    free_positions: Vec<usize>,
    total_positions: usize
}

impl Diskmap {
    fn new() -> Diskmap {
        Diskmap {
            files: Vec::new(),
            free_positions: Vec::new(),
            total_positions: 0
        }
    }

    fn load(&mut self, dm: &str) {
        let mut diskmap = dm.to_string();
        if diskmap.len() % 2 != 0 {
            diskmap.push('0');
        }
        for i in 0..diskmap.len() / 2 {
            let num_blocks = diskmap.chars().nth(i * 2).unwrap().to_digit(10).unwrap() as usize;
            let num_free = diskmap.chars().nth(i * 2 + 1).unwrap().to_digit(10).unwrap() as usize;
            let mut positions = Vec::new();
            for _ in 0..num_blocks {
                positions.push(self.total_positions);
                self.total_positions += 1;
            }
            for _ in 0..num_free {
                self.free_positions.push(self.total_positions);
                self.total_positions += 1;
            }
            self.files.push(Diskfile {
                id: i,
                positions
            });
        }
    }

    fn map_string(&self) -> String {
        let mut output = String::new();
        for i in 0..self.total_positions {
            if self.free_positions.contains(&i) {
                output.push('.');
            } else {
                let mut found = false;
                for file in &self.files {
                    if file.positions.contains(&i) {
                        output.push_str(&file.id.to_string());
                        found = true;
                        break;
                    }
                }
                if !found {
                    output.push('.');
                }
            }
        }
        output
    }

    fn defrag(&mut self) {
        for i in (0..self.files.len()).rev() {
            let mut positions = self.files[i].positions.clone();
            for i in (0..positions.len()).rev() {
                positions[i] = self.free_positions[0];
                self.free_positions.remove(0);
                if self.free_positions.len() == 0 {
                    return;
                }
            }
            self.files[i].positions = positions;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::Diskmap;

    fn test_one_string(input: &str, expected: &str) {
        let mut diskmap = Diskmap::new();
        diskmap.load(input);
        assert_eq!(diskmap.map_string(), expected);
    }

    #[test]
    fn test_12345() {
        test_one_string("12345", "0..111....22222");
    }

    #[test]
    fn test_92333133121414131402() {
        test_one_string("2333133121414131402", "00...111...2...333.44.5555.6666.777.888899");
    }

    fn test_one_defrag(input: &str, expected: &str) {
        let mut diskmap = Diskmap::new();
        diskmap.load(input);
        diskmap.defrag();
        assert_eq!(diskmap.map_string(), expected);
    }

    #[test]
    fn test_defrag_12345() {
        test_one_defrag("12345", "022111222......");
    }

    #[test]
    fn test_defrag_2333133121414131402() {
        test_one_defrag("2333133121414131402", "0099811188827773336446555566..............");
    }
}