pub fn part_one() -> usize{
  let mut count = 0;
  let mut max = 0;
  include_str!("inputs/day01.txt")
    .lines()
    .for_each(|n| {
      if n == "" {
        count = 0;
      } else {
        count += n.parse::<usize>().unwrap();
        if count > max {max = count;}
      }
    });
    max
}

pub fn part_two() -> usize {
  let mut count = 0;
  let mut max: Vec<usize> = Vec::new();
  max.push(0);
  include_str!("inputs/day01.txt")
    .lines()
    .for_each(|n| {
      if n == "" {
        max.push(count);
        max.sort_unstable_by(|a, b| b.cmp(a));
        if max.len() > 3 {
          max.pop();
        }
        count = 0;
      } else {
        count += n.parse::<usize>().unwrap();
      }
    });
    max.push(count);
    max.sort_unstable_by(|a, b| b.cmp(a));
    max.pop();
  max.iter().sum()
}