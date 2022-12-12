use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct State {
  distance: usize,
  position: usize,
  path: Vec<usize>,
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
      other
          .distance
          .cmp(&self.distance)
          .then_with(|| self.position.cmp(&other.position))
  }
}

pub fn part_one() -> usize{

  let mut start_node = 0;
  let mut target_node = 0;
  let mut start_coord = (0, 0);
  let mut target_coord = (0, 0);

  // parse the grid into a Vec<Vec<u32>> aliased as Grid<Row>
  let grid: Grid = include_str!("inputs/day12.txt")
    .lines()
    .enumerate()
    .map(|(y, l)| {
      l.bytes()
      .enumerate()
        .map(|(x, c)| {
          if c == 'S' as u8 { 
            start_coord = (x, y);
            'a' as usize
          }
          else if c == 'E' as u8 {
            target_coord = (x, y);
            'z' as usize
          }
          else { c as usize }
        }).collect()
    }).collect();


  // use Dijkstra to find the noooooode
  let height = grid.len();
  let width = grid[0].len();

  target_node = (target_coord.1 * width) + target_coord.0;
  start_node = (start_coord.1 * width) + start_coord.0;

  // find starting and ending nodes
  for y in 0..height {
    for x in 0..width {
      if grid[y][x] == 'S' as usize {start_node = y*width + x;}
      if grid[y][x] == 'E' as usize {target_node = y*width + x;}
    }
  }

  dbg!(start_node);
  dbg!(target_node);

  // populate dist with max usize value
  let mut dist: Vec<_> = (0..(height*width)).map(|_| usize::MAX).collect();

  let mut frontier = BinaryHeap::new();



  // set the distance to the starting node as 0 and push onto the frontier
  dist[start_node] = 0;
  frontier.push(
    State {
      distance: 0, 
      position: start_node,
      path: Vec::new(),
    }
  );

  while let Some(State { distance, position, mut path }) = frontier.pop() {

    // if the frontier is the target, we're done;
    if position == target_node {
      path.push(target_node);
      return Some(distance).unwrap();
    }

    // if there is a shorter distance to this position, skip
    if distance > dist[position] {
      continue;
    }

    // see if we can find a path with a lower cost than previous paths for any adjacent nodes
    // get list of neighbours for point
    let mut neighbors: Vec<usize> = Vec::new();
    // get x and y
    let x = position%width;
    let y = position/width;

    // determine the bounding edges
    let bound_top = y == 0;
    let bound_left = x == 0;
    let bound_bottom = y == (height-1);
    let bound_right = x == (width-1);

    let curr_value = grid[y][x];

    // dbg!(&grid);

    if !bound_top  {
      if (grid[y][x] - grid[y-1][x] + 1) < 1000 {
        neighbors.push(position-width);
      }
    }
    if !bound_bottom{
      if (grid[y][x] - grid[y+1][x] + 1) < 1000 {
        neighbors.push(position+width);
      }
      
    }
    if !bound_left {
      if (grid[y][x] - grid[y][x-1] + 1) < 1000 {
        neighbors.push(position-1);
      }
      
    }
    if !bound_right {
      if (grid[y][x] - grid[y][x+1] + 1) < 1000{
        neighbors.push(position+1);
      }
    }

    // check neighbors
    for neighbor in neighbors {

      let mut tmp_path = path.clone();
      tmp_path.push((y*width) + x);

      let next = State {
        // this line was causing problems, because it was adding the existing spot rather
        // than the neightbor position
        distance: distance + 1,
        position: neighbor,
        path: tmp_path,
      };
      if next.distance < dist[next.position] {
        // these must be reassigned to avoid using copy and the move error
        // if I want to keep Vec<usize> in the state to track the path
        let pos = next.position;
        let distance = next.distance;
        frontier.push(next);
        dist[pos] = distance;
      }
    }
  }
  0
}

pub fn part_two() -> usize{

  let mut start_node = 0;
  let mut start_coord = (0, 0);

  // parse the grid into a Vec<Vec<u32>> aliased as Grid<Row>
  let grid: Grid = include_str!("inputs/day12.txt")
    .lines()
    .enumerate()
    .map(|(y, l)| {
      l.bytes()
      .enumerate()
        .map(|(x, c)| {
          if c == 'E' as u8 {
            start_coord = (x, y);
            'z' as usize
          }
          else { c as usize }
        }).collect()
    }).collect();


  // use Dijkstra to find the noooooode
  let height = grid.len();
  let width = grid[0].len();

  start_node = (start_coord.1 * width) + start_coord.0;

  // find starting and ending nodes
  for y in 0..height {
    for x in 0..width {
      if grid[y][x] == 'E' as usize {start_node = y*width + x;}
    }
  }

  // populate dist with max usize value
  let mut dist: Vec<_> = (0..(height*width)).map(|_| usize::MAX).collect();

  let mut frontier = BinaryHeap::new();



  // set the distance to the starting node as 0 and push onto the frontier
  dist[start_node] = 0;
  frontier.push(
    State {
      distance: 0, 
      position: start_node,
      path: Vec::new(),
    }
  );

  while let Some(State { distance, position, mut path }) = frontier.pop() {

    let x = position%width;
    let y = position/width;

    // if the frontier is the target, we're done;
    if grid[y][x] == 'a' as usize {
      path.push(position);
      return Some(distance).unwrap();
    }

    // if there is a shorter distance to this position, skip
    if distance > dist[position] {
      continue;
    }

    // see if we can find a path with a lower cost than previous paths for any adjacent nodes
    // get list of neighbours for point
    let mut neighbors: Vec<usize> = Vec::new();
    // get x and y


    // determine the bounding edges
    let bound_top = y == 0;
    let bound_left = x == 0;
    let bound_bottom = y == (height-1);
    let bound_right = x == (width-1);

    let curr_value = grid[y][x];

    // dbg!(&grid);

    if !bound_top  {
      if (grid[y-1][x] - grid[y][x] + 1) < 1000 {
        neighbors.push(position-width);
      }
    }
    if !bound_bottom{
      if (grid[y+1][x] -grid[y][x] + 1) < 1000 {
        neighbors.push(position+width);
      }
      
    }
    if !bound_left {
      if (grid[y][x-1] - grid[y][x] + 1) < 1000 {
        neighbors.push(position-1);
      }
      
    }
    if !bound_right {
      if (grid[y][x+1] - grid[y][x] + 1) < 1000{
        neighbors.push(position+1);
      }
    }

    // check neighbors
    for neighbor in neighbors {

      let mut tmp_path = path.clone();
      tmp_path.push((y*width) + x);

      let next = State {
        // this line was causing problems, because it was adding the existing spot rather
        // than the neightbor position
        distance: distance + 1,
        position: neighbor,
        path: tmp_path,
      };
      if next.distance < dist[next.position] {
        // these must be reassigned to avoid using copy and the move error
        // if I want to keep Vec<usize> in the state to track the path
        let pos = next.position;
        let distance = next.distance;
        frontier.push(next);
        dist[pos] = distance;
      }
    }
  }
  0
}

type Row = Vec<usize>;
type Grid = Vec<Row>;