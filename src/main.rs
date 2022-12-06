#![feature(array_windows)]

use crate::solutions::*;
use std::env;
use took::took;

mod solutions;

macro_rules! solve {
  ($day:path) => {{
    use $day::*;
    
    let (took_1, result_1) = took(|| { part_one() });
    println!("Part 1: {}; \ttook: {}", result_1, took_1);
    let (took_2, result_2) = took(|| { part_two() });
    println!("Part 2: {}; \ttook: {}", result_2, took_2);
  }}
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let day: u8 = args[1].parse().unwrap();
  
  run_day(day);
}

fn run_day(day: u8) {
  println!("running day {}", day);
  match day {
    0 => solve!(day00),
    1 => solve!(day01),
    2 => solve!(day02),
    3 => solve!(day03),
    4 => solve!(day04),
    5 => solve!(day05),
    6 => solve!(day06),
    // new days go here
    _ => panic!("Something went wrong. '{}' was not a valid day.", day),
  }
}
