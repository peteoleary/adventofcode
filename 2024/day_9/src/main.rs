enum Block {
    File {id : usize},
    Free {}
}

#[derive(Clone)]
struct Diskfile {
    id: usize,
    positions: Vec<usize>
}

struct Diskmap {
    blocks: Vec<Block>
}

impl Diskmap {
    fn new() -> Diskmap {
        Diskmap {
            blocks: Vec::new()
        }
    }

    fn load(&mut self, dm: &str) {
        let mut diskmap = dm.to_string();
        if diskmap.len() % 2 != 0 {
            diskmap.push('0');
        }
        for id in 0..diskmap.len() / 2 {
            let num_blocks = diskmap.chars().nth(id * 2).unwrap().to_digit(10).unwrap() as usize;
            let num_free = diskmap.chars().nth(id * 2 + 1).unwrap().to_digit(10).unwrap() as usize;
            for _ in 0..num_blocks {
                self.blocks.push(Block::File {id});
            }
            for _ in 0..num_free {
                self.blocks.push(Block::Free {});
            }
        }
    }

    fn map_string(&self) -> String {
        let mut output = String::new();
        for block in &self.blocks {
            match block {
                Block::File {id} => output.push_str(&id.to_string()),
                Block::Free {} => output.push('.')
            }
        }
        output
    }

    fn find_first_free(&self) -> Option<usize> {
        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Free {} => return Some(i),
                _ => {}
            }
        }
        None
    }

    fn find_last_file(&self) -> Option<usize> {
        for (i, block) in self.blocks.iter().enumerate().rev() {
            match block {
                Block::File {id} => return Some(i),
                _ => {}
            }
        }
        None
    }

    fn defrag(&mut self) {
        while let (first, last) = (self.find_first_free(), self.find_last_file()) {
            match (first, last) {
                (Some(f), Some(l)) => {
                    if f > l {
                        break;
                    }
                    self.blocks.swap(f, l);
                },
                _ => break
            }
            
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