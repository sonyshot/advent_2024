use std::time::{SystemTime, UNIX_EPOCH};

mod puzzle_1;
mod puzzle_2;
mod puzzle_3;
// mod puzzle_4;
mod puzzle_5;
mod puzzle_8;

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{}", puzzle_8::solve());
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Executed in {} milliseconds", (end - start).as_millis());
}
