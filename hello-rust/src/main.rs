use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let mut found = false;
        let mut first = 0;
        let mut last = 0;
        let cv = String::from(line)
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "t3hree")
            .replace("four", "f4our")
            .replace("five", "f5ive")
            .replace("six", "s6ix")
            .replace("seven", "s7even")
            .replace("eight", "e8ight")
            .replace("nine", "n9ine");
        println!("Line: {cv}");

        for ch in cv.chars() {
            if ch.is_ascii_digit() {
                last = ch.to_digit(10).unwrap();
                if (!found) {
                    first = last;
                    found = true;
                }
            }
        }
        println!("first: {first} last: {last}");
        sum += first*10 + last;
    }
    println!("sum: {sum}");
    // let line = lines.next();
}
