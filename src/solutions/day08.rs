use std::collections::HashSet;

type Row = Vec<i32>;
type Grid = Vec<Row>;

pub fn part_one() -> usize{

  let mut visible: HashSet<i32> = HashSet::new();

  // parse the grid into a Vec<Vec<u32>> aliased as Grid<Row>
  let grid: Grid = include_str!("inputs/day08.txt")
    .lines()
    .map(|l| l.chars()
      .map(|c| c.to_digit(10).unwrap() as i32).collect()
    )
    .collect();

    let height: usize = grid.len();
    let width: usize = grid[0].len();

  // walk the edge of the grid
  let mut tmp: i32;
  for y in 0..height {
    tmp = -1;
    for x in 0..width {
      // coming in from the left
      if grid[y][x] > tmp {
        // add to set

        // println!("inserting {}, {}", x, y);
        visible.insert((y*width + x).try_into().unwrap());
        // set tmp to new grid
        tmp = grid[y][x];
      }
      if grid[y][x] == 9{
        break;
      }
    }
  }

  // println!("{:#?}", visible.iter());

  for y in 0..height {
    tmp = -1;
    for x in (0..width).rev() {
      // coming in from the right
      if grid[y][x] > tmp {
        // add to set
        visible.insert((y*width + x).try_into().unwrap());
        // set tmp to new grid
        tmp = grid[y][x];
      }
      if grid[y][x] == 9{
        break;
      }
    }
  }

  for x in 0..width {
    tmp = -1;
    for y in 0..height {
      // coming down from the top
      if grid[y][x] > tmp {
        // add to set
        visible.insert((y*width + x).try_into().unwrap());
        // set tmp to new grid
        tmp = grid[y][x];
      }
      if grid[y][x] == 9{
        break;
      }
    }
  }

  for x in 0..width {
    tmp = -1;
    for y in (0..height).rev() {
      // coming down from the top
      if grid[y][x] > tmp {
        // add to set
        visible.insert((y*width + x).try_into().unwrap());
        // set tmp to new grid
        tmp = grid[y][x];
      }
      if grid[y][x] == 9{
        break;
      }
    }
  }
  
  
  visible.iter().count()
}

pub fn part_two() -> i64 {

  // let mut visible: HashSet<i32> = HashSet::new();
  let mut scores: Vec<i64> = Vec::new();
  scores.push(0);

  // parse the grid into a Vec<Vec<u32>> aliased as Grid<Row>
  let grid: Grid = include_str!("inputs/day08.txt")
    .lines()
    .map(|l| l.chars()
      .map(|c| c.to_digit(10).unwrap() as i32).collect()
    )
    .collect();

    let height: usize = grid.len();
    let width: usize = grid[0].len();
  
  for y in 0..height {
    for x in 0..width {
      let mut score = 1i64;
      // checks all grid locations
      // fan out in all directions

      // check up

      if y > 0 {
        for y1 in (0..y).rev() {
          // check up
          if grid[y1][x] >= grid[y][x] || y1 == 0 {
            score = score * (y - y1) as i64;
            break;
          }
        }
      } else {score = 0;}

      // check down
      if y < height-1 {
        for y1 in (y+1)..height {
          // check down
          if grid[y1][x] >= grid[y][x]  || y1 == (height-1) {
            //blocked
            score = score * (y1 -y) as i64;
            break;
          }
        }
      } else {score = 0;}

      // check left
      if x > 0 {
        for x1 in (0..x).rev() {

          // check left
          if grid[y][x1] >= grid[y][x] || x1 == 0 {
            score = score * (x - x1) as i64;
            break;
          }
        }
      } else {score = 0;}

      // check right
      if x < width-1 {
        for x1 in (x+1)..width {
          // check right
          if grid[y][x1] >= grid[y][x] || x1 == (width - 1) {
            //blocked
            score = score * (x1 - x) as i64;
            break;
          }
        }
      } else {score = 0;}
      
      scores.push(score);
    }
  }
  *scores.iter().max().unwrap() as i64
}