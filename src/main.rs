#![allow(dead_code)]
use std::time::Instant;
mod utils;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let instant = Instant::now();
    println!("Day 1:");
    day_1::part_1_2();
    println!("Time: {}ms", instant.elapsed().as_millis());
    let instant = Instant::now();
    println!("Day 2:");
    day_2::part_1();
    println!("Time: {}ms", instant.elapsed().as_millis());
    let instant = Instant::now();
    day_3::part_1();
    println!("Time: {}ms", instant.elapsed().as_millis());

}
