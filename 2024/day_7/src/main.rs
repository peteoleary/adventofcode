use std::result;

use big_int::prelude::*;

mod binary_tree;

fn generate_bitmaps(n: usize) -> Vec<Vec<bool>> {
    let mut bitmaps = Vec::new();
    for i in 0..usize::pow(2, n as u32) {
        bitmaps.push(Vec::new());
        for j in 0..n {
            let bit = (i >> j) & 1;
            bitmaps[i].push(bit == 1);
        }
    }
    bitmaps
}

// this can be easily generalized into a function that generates for any base
fn generate_tripmaps(n: usize) -> Vec<Vec<u32>> {
    let mut tripmaps = Vec::new();
    let total_len = usize::pow(3, n as u32);
    let input: Vec<Tight<3>> = (0..total_len).map(|v| v.try_into().unwrap()).collect::<Vec<Tight<3>>>();
    for i in 0..total_len {
        tripmaps.push(Vec::new());
        let mut input_string = input[i].to_string();
        if input_string.len() < n {
            input_string = "0".repeat(n - input_string.len()) + &input_string;
        }
        input_string.chars().for_each(|c| {
            tripmaps[i].push(c.to_digit(10).unwrap());
        });
    }
    tripmaps
}

fn try_evals(nums: Vec<i32>, target: i64) -> bool {
    println!("try_evals: {} {:?}", target, nums);
    for operators in generate_tripmaps(nums.len() - 1) {
        let result: i64 = nums[1..].iter().zip(operators.iter()).fold(nums[0].into(), |acc, (num, op)| {
            if *op == 0 {
                acc + i64::from(*num)
            } else if *op == 1 {
                acc * i64::from(*num)
            } else {
                let mut new_string = acc.to_string();
                new_string.push_str((*num).to_string().as_str());
                new_string.parse::<i64>().unwrap()
                
            } 
        });
        if result == target {
            println!("found: {:?} {:?}", nums, target);
            return true;
        }
    }
    false
}

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let parts = line.split(':').collect::<Vec<_>>();
        let right_hand_side = parts[1].trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let target = parts[0].parse::<i64>().unwrap();
        if try_evals(right_hand_side, target) {
            sum += target;
        }
    }
    println!("sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_bitmaps_1() {
        let bitmaps = generate_bitmaps(1);
        assert_eq!(bitmaps, vec![
            vec![false],
            vec![true],
        ]);
    }
    #[test]
    fn test_generate_bitmaps_2() {
        let bitmaps = generate_bitmaps(2);
        assert_eq!(bitmaps, vec![
            vec![false, false],
            vec![true, false],
            vec![false, true],
            vec![true, true],
        ]);
    }
    #[test]
    fn test_generate_bitmaps_3() {
        let bitmaps = generate_bitmaps(3);
        assert_eq!(bitmaps, vec![
            vec![false, false, false],
            vec![true, false, false],
            vec![false, true, false],
            vec![true, true, false],
            vec![false, false, true],
            vec![true, false, true],
            vec![false, true, true],
            vec![true, true, true],
        ]);
    }
    #[test]
    fn test_try_evals() {
        assert_eq!(try_evals(vec![10, 19], 190), true);
    }
    #[test]
    fn test_generate_tripmaps_1() {
        let tripmaps = generate_tripmaps(1);
        assert_eq!(tripmaps, vec![
            vec![0],
            vec![1],
            vec![2]
        ]);
    }
    #[test]
    fn test_generate_tripmaps_2() {
        let tripmaps = generate_tripmaps(2);
        assert_eq!(tripmaps, vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 1],
            vec![1, 2],
            vec![2, 0],
            vec![2, 1],
            vec![2, 2],
        ]);
    }
}