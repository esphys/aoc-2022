pub fn part_one() -> isize{

  include_str!("inputs/day01.txt")
    .lines()
    .map(|n| n.parse::<i16>().unwrap())
    .collect::<Vec<i16>>()
    .array_windows()
    .filter(|[a, b]| a < b)
    .count();

  77
}

pub fn part_two() -> isize {
  let _input = include_str!("inputs/day01.txt").lines().map(|s| {s.parse::<isize>().unwrap()});
  75
}