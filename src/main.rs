use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
#[cfg(feature = "day7")]
mod d7;
mod d8;
pub mod intcode;


fn get_input(day: u8, suffix: &str) -> Vec<String> {
    let file_path = format!("resources/d{}{}.txt", day, suffix);
    let file = File::open(&file_path).expect(&format!("file: {} not found", file_path));
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.unwrap()).filter(|s| s.len() > 0).collect::<Vec<_>>()
}

fn main() {
    // d1::run();
    // d2::run();
    // d3::run();
    // d4::run();
    // d5::run();
    // d6::run();
    #[cfg(feature = "day7")]
    // d7::run();
    d8::run();
}
