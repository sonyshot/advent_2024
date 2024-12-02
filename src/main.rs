use std::time::{SystemTime, UNIX_EPOCH};

mod puzzle_1;
mod puzzle_2;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", puzzle_2::solve());
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Executed in {} milliseconds", (end - start).as_millis());
}
