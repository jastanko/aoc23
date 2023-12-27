use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"^([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)$").unwrap();
    
    let mut network = HashMap::new();
    let mut nodes = vec![];

    for hay in contents.lines() {
        for (_, [node, left, right]) in re.captures_iter(hay).map(|c| c.extract()) {
            network.insert(node, (left, right));
            if node.ends_with("A") {
                nodes.push(node);
            }  
        }
    }
    println!("found: {:?}", nodes);
    
    let path = "LRRRLRRLRLRRRLRLLLLRRRLRLRRLRLRLRRLRRRLRRLRRRLRLLLRRLRRLRRLRRLRRLLLLLRLRLRRRLRLLRRLRLRRRLRRLRRRLLLRRLRRLRRLRRRLRLRLRRLLRRRLRRLRRRLRRRLRRRLRLRRLRRLRRLRRRLRLRRLRRLLRRRLRRLRRLRRRLRLRLRRLLRRRLLRRLRRRLRRRLRLRRLLRRRLRLRRLLRRLRLRRRLRLRRRLRRLRRLRRLRRRLRRRLRLLRRLRRLLRRLRRRLRRLRLRLRRRLLLRRLRLRRLRRLRLRLLRLRRLRLRLRRRR";
    // let path = "LR";
    let mut counter = 0;

    'navigate: loop {
        for dir in path.chars() {
            for loc in &mut nodes {
                let next = network.get(loc).unwrap();
                if dir == 'L' {
                    *loc = next.0;
                } else {
                    *loc = next.1;
                }
            }
            counter += 1;
            // println!("{:?}", nodes);
            if nodes.iter().all(|s| s.ends_with("Z")) {
                break 'navigate;
            }
            if nodes.get(6).unwrap().ends_with("Z") {
                break 'navigate;
            }
        }
    }

    println!("Counter={counter} nodes={:?}", nodes);
}
