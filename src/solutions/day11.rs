#[derive(Debug)]
struct Monkey {
  items: Vec<usize>,
  operation: Operation,
  mod_test: usize,
  true_target: usize,
  false_target: usize,
  counter: usize,
}

#[derive(Debug)]
enum Operation {
  Add(usize),
  Multiply(usize),
  Square
}

impl Operation {
  fn execute(&self, x: usize) -> usize {
    match self {
      Operation::Add(x2) => x + x2,
      Operation::Multiply(x2) => x * x2,
      Operation::Square => x * x,
    }
  }
}

fn parse(input: &str) -> Vec<Monkey> {
  let mut monkeys: Vec<Monkey> = Vec::new();
  let mut lines = input.lines();

  while lines.next().is_some(){
    let starting_items = &lines.next().unwrap().trim()[16..];
    let operation_line = &lines.next().unwrap().trim()[21..];
    let test_line = &lines.next().unwrap().trim()[19..];
    let true_line = &lines.next().unwrap().trim()[25..];
    let false_line = &lines.next().unwrap().trim()[26..];
    let _ = &lines.next();

    let items: Vec<usize> = starting_items
    .split(',')
    .map(|x| x.trim().parse().unwrap())
    .collect();

    let operation = if operation_line == "* old" {
      Operation::Square
    } else if operation_line.starts_with('+') {
      Operation::Add(operation_line[2..].parse().unwrap())
    } else {
      Operation::Multiply(operation_line[2..].parse().unwrap())
    };

    let mod_test = test_line.parse().unwrap();
    let true_target = true_line.parse().unwrap();
    let false_target = false_line.parse().unwrap();

    let monkey = Monkey {
      items,
      operation,
      mod_test,
      true_target,
      false_target,
      counter: 0,
    };

    monkeys.push(monkey);
    
  }

  
  monkeys
}

pub fn part_one() -> usize{

  let mut monkeys: Vec<Monkey> = parse(include_str!("inputs/day11.txt"));

  const NUM_ROUNDS: usize = 20;

  for _ in 0..NUM_ROUNDS {
    for monkey_index in 0..monkeys.len() {
      while !monkeys[monkey_index].items.is_empty() {

        monkeys[monkey_index].counter += 1;

        let item = monkeys[monkey_index].items.remove(0);
        let item = monkeys[monkey_index].operation.execute(item) / 3;
        let next_monkey = if item % monkeys[monkey_index].mod_test == 0 {
          monkeys[monkey_index].true_target
        } else {
          monkeys[monkey_index].false_target
        };
        monkeys[next_monkey].items.push(item);
      }
    }
  }

  let mut counts = monkeys.into_iter().map(|monkey| {
    monkey.counter
  }).collect::<Vec<usize>>();
  counts.sort();

  // dbg!(&counts);

  let top_two = counts.into_iter().rev().take(2);
  top_two.product()
}

pub fn part_two() -> usize {
  let mut monkeys: Vec<Monkey> = parse(include_str!("inputs/day11.txt"));
  
  const NUM_ROUNDS: usize = 10000;

  let mod_product = monkeys.iter().map(|x| x.mod_test).product::<usize>();

  for _ in 0..NUM_ROUNDS {
    for monkey_index in 0..monkeys.len() {
      while !monkeys[monkey_index].items.is_empty() {

        monkeys[monkey_index].counter += 1;

        let item = monkeys[monkey_index].items.remove(0);
        let item = monkeys[monkey_index].operation.execute(item) % mod_product;
        let next_monkey = if item % monkeys[monkey_index].mod_test == 0 {
          monkeys[monkey_index].true_target
        } else {
          monkeys[monkey_index].false_target
        };
        monkeys[next_monkey].items.push(item);
      }
    }
  }

  let mut counts = monkeys.into_iter().map(|monkey| {
    monkey.counter
  }).collect::<Vec<usize>>();
  counts.sort();
  let top_two = counts.into_iter().rev().take(2);
  top_two.product()
}