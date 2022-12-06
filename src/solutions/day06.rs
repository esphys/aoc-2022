// use std::fs;

pub fn part_one() -> usize{
  let input = include_str!("inputs/day06.txt");
  // let input = fs::read_to_string("src/solutions/inputs/day06.txt").unwrap();
  slide_window(input, 4)
}

pub fn part_two() -> usize {
  let input = include_str!("inputs/day06.txt");
  // let input = fs::read_to_string("src/solutions/inputs/day06.txt").unwrap();
  slide_window(input, 14)
}

fn slide_window(input: &str, size: usize) -> usize {
  for i in 0..=(input.len()-size){
    if has_duplicates(&input[i..(i+size)]) {
      return i + size;
    }
  }
  0
}

fn has_duplicates(window: &str) -> bool {
  let mut mask = 0u32;
  for i in 0..window.len() {
    let tmp_mask = 1u32 << (window.as_bytes()[i] - 'a' as u8);
    if tmp_mask & mask > 0 {
      return false;
    } else {
      mask = mask | tmp_mask;
    }
  }
  true
}