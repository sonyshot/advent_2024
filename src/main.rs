use std::time::{SystemTime, UNIX_EPOCH};

mod puzzle_1;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", puzzle_1::solve());
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Executed in {} milliseconds", (end - start).as_millis());
}
