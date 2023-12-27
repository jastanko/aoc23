use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum: i64 = 0;
    let len = 100;
      
    let mut col = 0;
    while col < len {
        let mut row = 100;
        let mut load = 100;
            for line in contents.lines() {
            let ch = line.chars().nth(col).unwrap();
            if ch == 'O' {
                sum += load;
                load -= 1;
            } else if ch == '#' {
                load = row - 1;
            }
            row -= 1;
        }
        col += 1;            
    }
    println!("sum={sum}");    
}
