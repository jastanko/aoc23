use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut galaxies = Vec::new();
    let expansion = 999999;
    
    let mut row: i64 = 0;
    let mut col: i64 = 0;
    for line in contents.lines() {
        row = 0;
        for ch in line.chars() {
            if ch == '#' {
                galaxies.push((row, col));
            }
            row += 1;
        }
        col += 1;
    }

    // println!("{:?}", galaxies);
    while col > 0 {
        col -= 1;
        if !galaxies.iter().any(|x| x.1 == col) {
            // println!("No galaxies in column {col}");
            for g in &mut galaxies {
                if g.1 > col {
                    g.1 += expansion;
                }
            }
        }
    }
    // println!("{:?}", galaxies);

    while row > 0 {
        row -= 1;
        if !galaxies.iter().any(|x| x.0 == row) {
            for g in &mut galaxies {
                if g.0 > row {
                    g.0 += expansion;
                }
            }
        }
    }
    // println!("{:?}", galaxies);

    let mut sum: u64 = 0;
    let mut i = 0;
    while i < galaxies.len() {
        let mut j = i + 1;
        while j < galaxies.len() {
            sum += distance(galaxies.get(i).unwrap(), galaxies.get(j).unwrap());
            j += 1;
        }
        i += 1;
    }
    println!("Sum: {sum}");
}

fn distance(a: &(i64, i64), b: &(i64, i64)) -> u64 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}