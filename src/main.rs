#![feature(array_windows)]

use crate::solutions::*;
use std::env;
use took::took;

mod solutions;

macro_rules! solve {
  ($day:path) => {{
    use $day::*;

    // bigboy x 10000
    // let (bigboi_took_1, bigboi_result_1) = took(|| { 
    //   let mut result = 0;
    //   for _ in 0..10000 {
    //     result = part_one();
    //   }
    //   result
    // });
    // println!("bigboy part_one: {}; \ttook: {}", bigboi_result_1, bigboi_took_1);

    // let (bigboi_took_1, bigboi_result_1) = took(|| { 
    //   let mut result = 0;
    //   for _ in 0..10000 {
    //     result = part_two();
    //   }
    //   result
    // });
    // println!("bigboy part_two: {}; \ttook: {}", bigboi_result_1, bigboi_took_1);


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
    15 => solve!(day15),
    13 => solve!(day13),
    3 => solve!(day03),
    4 => solve!(day04),
    5 => solve!(day05),
    // new days go here
    _ => panic!("Something went wrong. '{}' was not a valid day.", day),
  }
}
