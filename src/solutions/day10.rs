pub fn part_one() -> i32{

  let mut x: i32 = 1;
  let mut counter: u32 = 0;
  let mut signal_strength: i32 = 0;

  // noop add 1 to counter
  // add add to x and +2 to counter

  include_str!("inputs/day10.txt")
    .lines().for_each(|instruction| {
      match instruction {
        i if i.starts_with("noop") => {
          // println!("noop")
          counter += 1;
          signal_strength += strength(counter, x);
        },
        i if i.starts_with("addx") => {
          // println!("addx")
          counter += 1;
          signal_strength += strength(counter, x);
          // check signal strength
          counter += 1;
          signal_strength += strength(counter, x);
          x += instruction.split_once(' ').unwrap().1.parse::<i32>().unwrap();
          // check signal strength
        },
        _ => println!("unknown instruction")
      }
    });
  signal_strength
}

fn strength(counter: u32, register: i32) -> i32 {
  match counter {
    20 | 60 | 100 | 140 | 180 | 220 => {
      let strength = (i64::from(counter) * i64::from(register)).try_into().unwrap();
      // println!("counter: {}, register: {}, strength = {}", counter, register, strength);
      return strength
    },
    _ => 0
  }
}

pub fn part_two() -> String {

  let mut display = ['.'; 240];

  let mut x: i32 = 1;
  let mut counter: u32 = 0;
  let mut signal_strength: i32 = 0;

  include_str!("inputs/day10.txt")
  .lines().for_each(|instruction| {
    match instruction {
      i if i.starts_with("noop") => {



        // println!("noop")
        counter += 1;
        // drawing logic:
        for p in -1..=1 {
          // check 3 pixels
          if <u32 as TryInto<i32>>::try_into((counter-1)%40).unwrap() == (x+p) {
            // this means a pixel is being drawn in theory
            display[usize::try_from(counter).unwrap()-1] = '#';
          }
        }
        


        //
        signal_strength += strength(counter, x);
      },
      i if i.starts_with("addx") => {
        // println!("addx")
        counter += 1;
        for p in -1..=1 {
          // check 3 pixels
          if <u32 as TryInto<i32>>::try_into((counter-1)%40).unwrap() == (x+p) {
            // this means a pixel is being drawn in theory
            display[usize::try_from(counter).unwrap()-1] = '#';
          }
        }
        signal_strength += strength(counter, x);
        // check signal strength
        counter += 1;
        for p in -1..=1 {
          // check 3 pixels
          if <u32 as TryInto<i32>>::try_into((counter-1)%40).unwrap() == (x+p) {
            // this means a pixel is being drawn in theory
            display[usize::try_from(counter).unwrap()-1] = '#';
          }
        }
        signal_strength += strength(counter, x);
        x += instruction.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        // check signal strength
      },
      _ => println!("unknown instruction")
    }
  });

  // draw_screen(&display);
  get_output(&display)
}

fn _draw_screen(buffer: &[char]) {
  const WIDTH: usize = 40;
  const HEIGHT: usize = 6;
  for y in 0..HEIGHT {
    for x in 0..WIDTH {
      print!("{} ", buffer[y*WIDTH + x]);
    }
    print!("\n");
  }
}

fn get_output(buffer: &[char]) -> String {
  // each letter pattern stored as a u32
  // let alphabet: HashMap<u32, char> = HashMap::from([
  //   (0b011001001010010111101001010010, 'A'),
  //   (0b111001001011100100101001011100, 'B'),
  //   (0b011001001010000100001001001100, 'C'),
  //   (0b111001001010010100101001011100, 'D'),
  //   (0b111101000011100100001000011110, 'E'),
  //   (0b111101000011100100001000010000, 'F'),
  //   (0b011001001010000101101001001110, 'G'),
  //   (0b100101001011110100101001010010, 'H'),
  //   (0b111000100001000010000100011100, 'I'),
  //   (0b001100001000010000101001001100, 'J'),
  //   (0b100101010011000101001010010010, 'K'),
  //   (0b100001000010000100001000011110, 'L'),
  //   (0b100101111011110100101001010010, 'M'),
  //   (0b100101101011010101101011010010, 'n'),
  //   (0b011001001010010100101001001100, 'O'),
  //   (0b111001001010010111001000010000, 'P'),
  //   (0b011001001010010100100110000010, 'Q'),
  //   (0b111001001010010111001010010010, 'R'),
  //   (0b011001001001000001001001001100, 'S'),
  //   (0b011100010000100001000010000100, 'T'),
  //   (0b100101001010010100101001001100, 'U'),
  //   (0b100101001010010100100110001100, 'V'),
  //   (0b100101001010010111101111010010, 'W'),
  //   (0b100101001001100011001001010010, 'X'),
  //   (0b101001010010100010000100001000, 'Y'),
  //   (0b111100001000100010001000011110, 'Z')
  // ]);

  let alphabet: [u32; 26] = [
    0b011001001010010111101001010010,
    0b111001001011100100101001011100,
    0b011001001010000100001001001100,
    0b111001001010010100101001011100,
    0b111101000011100100001000011110,
    0b111101000011100100001000010000,
    0b011001001010000101101001001110,
    0b100101001011110100101001010010,
    0b111000100001000010000100011100,
    0b001100001000010000101001001100,
    0b100101010011000101001010010010,
    0b100001000010000100001000011110,
    0b100101111011110100101001010010,
    0b100101101011010101101011010010,
    0b011001001010010100101001001100,
    0b111001001010010111001000010000,
    0b011001001010010100100110000010,
    0b111001001010010111001010010010,
    0b011001001001000001001001001100,
    0b011100010000100001000010000100,
    0b100101001010010100101001001100,
    0b100101001010010100100110001100,
    0b100101001010010111101111010010,
    0b100101001001100011001001010010,
    0b101001010010100010000100001000,
    0b111100001000100010001000011110
  ];

  let length = buffer.len()/6;
  let height = 6;
  let char_width = 5;

  let mut result: Vec<char> = Vec::new();
  for k in 0..(length/char_width) {
    let mut mask: u32 = 0b0;
    for i in 0..height {
      let line = &buffer[((length*i) + (char_width * k))..((length*i) + (char_width * k)) + char_width];
      for (j, c) in line.iter().enumerate() {
        if c == &'#' {
          mask = mask | (0b1  << (29 - (j + (i*char_width))));
        }
      }
    }
    let ascii: u8 = (alphabet.iter().position(|x| x == &mask).unwrap() + ('A' as usize)).try_into().unwrap();
    result.push(ascii as char);

  }
  result.into_iter().collect::<String>()
}

