use std::collections::VecDeque;

pub fn part_one() -> String{
  let mut stack_width = 0;
  let mut stacks: Vec<VecDeque<char>> = Vec::new();
  include_str!("inputs/day05.txt")
    .lines().for_each(|line| {
      if line.is_empty() {}
      else if !line.starts_with('m') {
        if line.starts_with(" 1") {}
        else {
          // parse the nightmare
          if stacks.is_empty() {
            // initialize the correct number of stacks
            stack_width = (line.len()+1)/4;
            for _ in 0..stack_width {
              stacks.push(VecDeque::new());
            }
          }
          let chars = line.as_bytes();

          for (i, stack) in stacks.iter_mut().enumerate() {
            if chars[(i*4)+1] != ' ' as u8 {
              // put a thing in the stack;
              stack.push_front(chars[(i*4)+1] as char);
            }
          }
        }
        // println!("parsing map");
      } else {
        // do the moves
        let mut instruction = line.split(' ');
        
        let moves = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        let from = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        let to = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        
        for _ in 0..moves {
          let krate = stacks.iter_mut().nth(from-1).unwrap().pop_back().unwrap();
          stacks.iter_mut().nth(to-1).unwrap().push_back(krate);
        }
      }
    });
    
    // collect the tops of each stack;
    // dbg!(stacks);
    let mut top: Vec<char> = Vec::new();
    stacks.iter_mut().for_each(|stack| {
      top.push(stack.pop_back().unwrap());
    });
  top.iter().collect::<String>()
}

pub fn part_two() -> String{
  let mut stack_width = 0;
  let mut stacks: Vec<VecDeque<char>> = Vec::new();
  include_str!("inputs/day05.txt")
    .lines().for_each(|line| {
      if line.is_empty() {}
      else if !line.starts_with('m') {
        if line.starts_with(" 1") {}
        else {
          // parse the nightmare
          if stacks.is_empty() {
            // initialize the correct number of stacks
            stack_width = (line.len()+1)/4;
            for _ in 0..stack_width {
              stacks.push(VecDeque::new());
            }
          }
          let chars = line.as_bytes();

          for (i, stack) in stacks.iter_mut().enumerate() {
            if chars[(i*4)+1] != ' ' as u8 {
              // put a thing in the stack;
              stack.push_front(chars[(i*4)+1] as char);
            }
          }
        }
        // println!("parsing map");
      } else {
        // do the moves
        let mut instruction = line.split(' ');
        
        let moves = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        let from = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        let to = instruction.nth(1).unwrap().parse::<usize>().unwrap();
        let mut crane: Vec<char> = Vec::new();
        for _ in 0..moves {
          let krate = stacks.iter_mut().nth(from-1).unwrap().pop_back().unwrap();
          crane.push(krate);
        }
        for _ in 0..moves {
          stacks.iter_mut().nth(to-1).unwrap().push_back(crane.pop().unwrap());
        }
      }
    });
    
    // collect the tops of each stack;
    // dbg!(stacks);
    let mut top: Vec<char> = Vec::new();
    stacks.iter_mut().for_each(|stack| {
      top.push(stack.pop_back().unwrap());
    });
  top.iter().collect::<String>()
}