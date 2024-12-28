use std::{collections::HashMap, hash::Hash};
use petgraph::graphmap::GraphMap;

type Dag = HashMap<u32, Vec<u32>>;

// return true if there exists a path from start to end going only in the forward direction
fn is_before(dag: &Dag, start: u32, end: u32) -> bool {
    match dag.get(&start) {
        Some(value) => {
            value.contains(&end)
        },
        None => false
    }
}

// return true if head is before all elements in tail
fn is_before_list(dag: &Dag, before: u32, after: Vec<u32>) -> bool {
    // println!("Checking if {} is before {:?}", before, after);
    for node in after {
        if !is_before(dag, before, node) {
            return false;
        }
    }
    return true;
}

fn is_after_list(dag: &Dag, after: u32, before: Vec<u32>) -> bool {
    // println!("Checking if {} is after {:?}", after, before);
    for node in before {
        if !is_before(dag, node, after) {
            return false;
        }
    }
    return true;
}

fn correct_updates(dag: &Dag, updates: &Vec<u32>) -> bool {
    for i in 0..updates.len() {
        let current = updates[i];
        if !is_before_list(dag, current, updates[i+1..].to_vec()) {
            return false;
        }
        if !is_after_list(dag, current, updates[..i].to_vec()) {
            return false;
        }
    }
    return true;
}

fn main() {
    let input = std::fs::read_to_string("src/big_input.txt").unwrap();
    let lines = input.lines();
    println!("Hello, world!");

    let mut reading_edges = true;
    let mut dag= Dag::new();
    let mut correct_sum = 0;
    let mut incorrect_sum = 0;
    for line in lines {
        if line.len() == 0 {
            reading_edges = false;
            continue;
        }
        if reading_edges {
            let parts: Vec<&str> = line.split("|").collect();
            let parent: u32 = parts[0].parse().unwrap();
            let child: u32 = parts[1].parse().unwrap();
            if dag.contains_key(&parent) {
                dag.get_mut(&parent).unwrap().push(child);
            } else {
                dag.insert(parent, vec![child]);
            }
        } else { // reading updates
            let mut updates = line.split(",").map(|x| x.parse().unwrap()).collect();
            if correct_updates(&dag, &updates) {
                println!("Correct updates: {:?}", updates);
                correct_sum += updates[updates.len() / 2]
            } else {
                updates.sort_by(|a, b| {
                    if is_before(&dag, *a, *b)
                    {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                println!("Incorrect updates: {:?}", updates);
                incorrect_sum += updates[updates.len() / 2]
            }
        }
    }
    println!("Correct Sum: {}", correct_sum);
    println!("Incorrect Sum: {}", incorrect_sum);
}
