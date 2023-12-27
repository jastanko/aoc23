fn main() {
    // let ways =    
    //     ways_to_win(59, 430)*
    //     ways_to_win(70, 1218)*
    //     ways_to_win(78, 1213)*
    //     ways_to_win(78, 1276);
    let ways = ways_to_win(59707878, 430121812131276);
    println!("Ways to win: {ways}");
}

fn ways_to_win(time: i64, record: i64) -> i32 {
    let mut ways = 0;
    
    for n in 1..time {
        let dist = n*(time-n);
        // println!("time: {n} dist: {dist}");
        if dist > record {
            ways += 1;
        }
    }
    ways
}