use std::fs;

fn main() {
    let file_path = String::from("./input.txt");
    println!("In file {}", file_path);

    let steps = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    // let steps = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let mut sum: u64 = 0;

    for step in steps.split(',') {
        let mut result: u64 = 0;
        for ch in step.chars() {
            if !ch.is_control() {
                let code: u64 = ch.into();
                result = (result + code) * 17 % 256;
            }
        }
        // println!("Step: {step} Sum: {result}");
        sum += result;    
    }

    println!("Sum: {sum}");
}
