use std::fs;

fn main() {
    let file_path = String::from("./small.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut hands = Vec::new();
    
    for line in contents.lines() {
        let hand = line.split_once(' ').unwrap();

        let mut cards: Vec<char> = hand.0.chars().into_iter().collect();
        cards.sort();
        hands.push(cards);
    }

    hands.sort();
    println!("{:?}", hands);
}