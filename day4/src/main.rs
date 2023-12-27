use std::{fs, collections::HashSet};

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total = 0;
    let mut total_cards = 0;
    let mut copy_counters: Vec<(u32, u32)> = Vec::new();
    for line in contents.lines() {
        let card = line.split_once(':').unwrap().1.split_once('|').unwrap();
        // let mut winning = card.0.trim().split(' ');
        let numbers = card.1.trim().split(' ');

        let mut winners: HashSet<i32> = HashSet::new();
        for num in card.0.trim().split(' ') {
            let n = num.parse();
            if n.is_ok() {
                winners.insert(n.unwrap());
            }
        }
        let mut matches = 0;
        for num in numbers {
            let n = num.parse();
            let winner = n.is_ok() && winners.contains(&n.unwrap());
            if winner {
                matches += 1;
                // print!("{num},");
            }
        }

        let copies = copy_counters.iter().map(|c| c.0).sum::<u32>() + 1;

        // run down the counters
        copy_counters = copy_counters.iter().map(|c| (c.0, c.1-1)).filter(|c| c.1 > 0).collect();

        if matches > 0 {
            total += 2_i32.pow(matches - 1);
            copy_counters.push((copies, matches));
        }
        println!("copies: {copies}");
        total_cards += copies;
    }
    println!("Total: {total}, Total cards: {total_cards}");
}
