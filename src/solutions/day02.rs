pub fn part_one() -> usize{
  include_str!("inputs/day02.txt")
    .lines()
    .map(|s| {
      let bytes = s.as_bytes();
      let you = bytes[2] as char;
      let them = bytes[0] as char;
      match you {
        'X' => {
          match them {
            'A' => 4,
            'B' => 1,
            'C' => 7,
            _ => 0
          }
        },
        'Y' => {
          match them {
            'A' => 8,
            'B' => 5,
            'C' => 2,
            _ => 0
          }
        },
        'Z' => {
          match them {
            'A' => 3,
            'B' => 9,
            'C' => 6,
            _ => 0
          }
        },
        _ => 0
      }
    }).sum()
}

pub fn part_two() -> usize {
  include_str!("inputs/day02.txt")
    .lines()
    .map(|s| {
      let bytes = s.as_bytes();
      let you = bytes[2] as char;
      let them = bytes[0] as char;
        match you {
          'X' => {
            match them {
              'A' => 3,
              'B' => 1,
              'C' => 2,
              _ => 0
            }
          },
          'Y' => {
            match them {
              'A' => 4,
              'B' => 5,
              'C' => 6,
              _ => 0
            }
          },
          'Z' => {
            match them {
              'A' => 8,
              'B' => 9,
              'C' => 7,
              _ => 0
            }
          },
          _ => 0
        }
    }).sum()
}