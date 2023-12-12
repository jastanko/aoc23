use std::fs;

fn main() {
    let file_path = String::from("./small.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut symbols = Vec::new();
      
    let mut y: i32 = 0;
    for line in contents.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            if !(ch == '.' || ch.is_ascii_digit()) {
                symbols.push((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    let mut sum = 0;
    y = 0;
    for line in contents.lines() {
        let mut x: i32 = 0;
        let mut found = false;
        let mut part_num = 0;
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                part_num = part_num*10 + ch.to_digit(10).unwrap();
                found = found || symbols.iter().any(|&loc| x.abs_diff(loc.0) < 2 && y.abs_diff(loc.1) < 2);
            } else {
                if found {
                    // println!("found part: {part_num}");
                    sum += part_num;
                }
                part_num = 0;
                found = false;
            }
            x += 1;
        }
        if found {
            // println!("found part: {part_num}");
            sum += part_num;
        }
        y += 1;
    }
    println!("sum={sum}");    
}
