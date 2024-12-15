use std::{collections::HashMap, thread::panicking};

use petgraph::graphmap::GraphMap;

// return true if there exists a path from start to end going only in the forward direction
fn is_before(dag: &GraphMap<u32, (), petgraph::Directed>, start: u32, end: u32) -> bool {
    let mut visited: HashMap<u32, bool> = HashMap::new();
    let mut stack: Vec<u32> = vec![start];
    while stack.len() > 0 {
        let node = stack.pop().unwrap();
        if node == end {
            return true;
        }
        if visited.contains_key(&node) {
            panic!("Cycle detected");
        }
        else {        
            visited.insert(node, true);
            for neighbor in dag.neighbors_directed(node, petgraph::Direction::Outgoing) {
                stack.push(neighbor);
            }
        }
    }
    return false;
}

// return true if head is before all elements in tail
fn is_before_list(dag: &GraphMap<u32, (), petgraph::Directed>, before: u32, after: Vec<u32>) -> bool {
    // println!("Checking if {} is before {:?}", before, after);
    for node in after {
        if !is_before(dag, before, node) {
            return false;
        }
    }
    return true;
}

fn is_after_list(dag: &GraphMap<u32, (), petgraph::Directed>, after: u32, before: Vec<u32>) -> bool {
    // println!("Checking if {} is after {:?}", after, before);
    for node in before {
        if !is_before(dag, node, after) {
            return false;
        }
    }
    return true;
}

fn correct_updates(dag: &GraphMap<u32, (), petgraph::Directed>, updates: &Vec<u32>) -> bool {
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
    let mut dag: GraphMap<u32, (), petgraph::Directed> = GraphMap::<u32, (), petgraph::Directed>::new();
    let mut sum = 0;
    for line in lines {
        if line.len() == 0 {
            reading_edges = false;
            continue;
        }
        if reading_edges {
            let parts: Vec<&str> = line.split("|").collect();
            let parent: u32 = parts[0].parse().unwrap();
            let child: u32 = parts[1].parse().unwrap();
            dag.add_node(parent);
            dag.add_node(child);
            dag.add_edge(parent, child, ());
        } else { // reading updates
            let updates = line.split(",").map(|x| x.parse().unwrap()).collect();
            if correct_updates(&dag, &updates) {
                println!("Correct updates: {:?}", updates);
                sum += updates[updates.len() / 2]
            } else {
                println!("Incorrect updates");
            }
        }
    }
    print!("Sum: {}", sum);
}
