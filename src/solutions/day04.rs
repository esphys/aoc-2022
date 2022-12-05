pub fn part_one() -> usize{
  include_str!("inputs/day04.txt")
    .lines()
    .filter(|line| {
      let pair: Vec<Vec<usize>> = line.split(",")
        .map(|ids| {
          ids.split("-").map(|n| n.parse::<usize>().unwrap()).collect()
        }).collect();
      (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) 
      || (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1])
    }).count()
}

pub fn part_two() -> usize {
  include_str!("inputs/day04.txt")
  .lines()
  .filter(|line| {
    let pair: Vec<Vec<usize>> = line.split(",")
      .map(|ids| {
        ids.split("-").map(|n| n.parse::<usize>().unwrap()).collect()
      }).collect();
    let a = (pair[0][0]..=(pair[0][1])).collect::<Vec<usize>>();
    let b = (pair[1][0]..=(pair[1][1])).collect::<Vec<usize>>();
    a.iter().filter(|n| {
      b.contains(n)
    }).count() > 0
  }).count()
}