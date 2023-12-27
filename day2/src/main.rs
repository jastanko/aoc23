use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut id = 1;
    let mut total = 0;
    for line in contents.lines() {
        let game = line.split_once(':').unwrap_or_default().1;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in game.split(';') {
            for cube in subset.split(',') {
                let pair = cube.trim().split_once(' ').unwrap();
                let n: i32 = pair.0.parse().unwrap();
                if pair.1 == "red" && n > red {
                    red = n;
                }
                if pair.1 == "green" && n > green {
                    green = n;
                }
                if pair.1 == "blue" && n > blue {
                    blue = n;
                }
            }
        }
        let power = red * green * blue;
        // println!("Game {id}: {power}");
        total += power;
        id += 1;
    }
    println!("Total: {total}");
}
