use std::collections::HashSet;

pub fn part_one() -> usize{

  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  let mut head = (0i32, 0i32);
  let mut tail = (0i32, 0i32);

  include_str!("inputs/day09.txt")
    .lines().for_each(|line| {
      let dir = line.split_once(' ').unwrap().0;
      let count = line.split_once(' ').unwrap().1.parse::<i32>().unwrap();

      for _ in 0..count {
        match dir {
          "U" => {
            head.1 += 1;
            if !is_following(head, tail) {
              catch_up_2(head, &mut tail);
            }
          },
          "R" => {
            head.0 += 1;
            if !is_following(head, tail) {
              catch_up_2(head, &mut tail);
            }
            
          },
          "D" => {
            head.1 -= 1;
            if !is_following(head, tail) {
              catch_up_2(head, &mut tail);
            }
          },
          "L" => {
            head.0 -= 1;
            if !is_following(head, tail) {
              catch_up_2(head, &mut tail);
            }
          },
          _ => {}
        }
        visited.insert(tail);
      }
    });
  visited.iter().count()
}

pub fn part_two() -> usize {
  let mut visited: HashSet<(i32, i32)> = HashSet::new();
  let mut head = (0i32, 0i32);
  let mut tails = vec![(0i32, 0i32); 9];

  include_str!("inputs/day09.txt")
    .lines().for_each(|line| {
      let dir = line.split_once(' ').unwrap().0;
      let count = line.split_once(' ').unwrap().1.parse::<i32>().unwrap();

      for _ in 0..count {
        match dir {
          "U" => {
            head.1 += 1;
          },
          "R" => {
            head.0 += 1;
          },
          "D" => {
            head.1 -= 1;
          },
          "L" => {
            head.0 -= 1;
          },
          _ => {}
        }
        let mut tmp_head = head;
        tails.iter_mut().for_each(|tail| {
            if !is_following(tmp_head, *tail) {
              catch_up_2(tmp_head, tail);
            }
          tmp_head = *tail;
        });
        visited.insert(tails.last().unwrap().clone());
      }
    });

    // print out the final grid
    // println!("Head position: ({}, {}); \tTail position: ({}, {})", head.0, head.1, tails.last().unwrap().0, tails.last().unwrap().1);
    // for y in (-20..20).rev() {
    //   for x in -20..20 {
    //     if head == (x, y) {print!("H");}
    //     else if visited.contains(&(x, y)) {
    //       print!("#");
    //     }
    //     else {print!(".");}
    //   }
    //   println!("");
    // }
    // println!("");

  visited.iter().count()
}

fn is_following(head: (i32, i32), tail: (i32, i32)) -> bool {
  (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn catch_up_2(head: (i32, i32), tail: &mut (i32, i32)) {
  let delta = (head.0-tail.0, head.1-tail.1);
  if delta.1.abs() > 1 && delta.1.abs() > delta.0.abs()  {
    tail.0 = head.0;
    if head.1 < tail.1 {
      tail.1 = head.1+1;
    } else {
      tail.1 = head.1-1;
    }
  }
  else if delta.0.abs() > 1 && delta.1.abs() < delta.0.abs(){
    tail.1 = head.1;
    if head.0 < tail.0 {
      tail.0 = head.0+1;
    } else {
      tail.0 = head.0-1;
    }
  } else {
    if head.0 > tail.0 {
      tail.0 = head.0-1;
    } else if head.0 < tail.0 {
      tail.0 = head.0+1;
    }

    if head.1 > tail.1 {
      tail.1 = head.1-1;
    } else if head.1 < tail.1 {
      tail.1 = head.1+1;
    }
  }
}
