use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut first = true;
    let mut seeds: Vec<(i64, bool)> = Vec::new();
    // for seed in contents.lines().next().unwrap().strip_prefix("seeds: ").unwrap().split_ascii_whitespace() {
    //     seeds.push((seed.parse().unwrap(), false));
    // }
    
    for line in contents.lines() {
        if first {
            seeds = line.strip_prefix("seeds: ").unwrap().split_ascii_whitespace().into_iter().map(|x| (x.parse().unwrap(), false)).collect();
            first = false;
            println!("{:?}", seeds);
        } else {
            if line.chars().any(|c| c.is_ascii_digit()) {
                let map: Vec<i64> = line.split_ascii_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();
                let dest = map.get(0).unwrap();
                let source = map.get(1).unwrap();
                let range = map.get(2).unwrap();
                let max = source + range;
                let delta = dest - source;

                for seed in &mut seeds {
                    if !seed.1 && seed.0 >= *source && seed.0 < max {
                        *seed = (seed.0 + delta, true);
                    }
                    
                }
                // println!("{dest} {source}-{max} {range} {delta} {:?}", seeds);
            } else {
                for seed in &mut seeds {
                    seed.1 = false;
                }
            }
        }
    }
    let min = seeds.iter().map(|x| x.0).min().unwrap();
    println!("Minimum:{min}");
}
