pub fn part_one() -> u32{
  let mut mask_1 = 0u64;
  let mut mask_2 = 0u64;
  include_str!("inputs/day03.txt")
    .lines()
    .map(|elf| {
      mask_1 = 0u64;
      let _ = &elf[..elf.len()/2].as_bytes().iter().for_each(|item| {
        mask_1 = mask_1 | convert_to_mask(&(item));
      });
      mask_2 = 0u64;
      let _ = &elf[elf.len()/2..].as_bytes().iter().for_each(|item| {
        mask_2 = mask_2 | convert_to_mask(item);
      });
      (mask_1 & mask_2).trailing_zeros()
    }).sum()
}

pub fn part_two() -> u32 {
  let mut mask_accum = 0u64;
  let mut mask_tmp = 0u64;
  include_str!("inputs/day03.txt")
    .lines().collect::<Vec<&str>>()
    .chunks(3).map(|group| {
      mask_accum = u64::MAX;
      group.iter().for_each(|elf| {
        mask_tmp = 0u64;
        elf.as_bytes().iter().for_each(|item| {
          mask_tmp = mask_tmp | convert_to_mask(item);
        });
        mask_accum = mask_accum & mask_tmp;
      });
      mask_accum.trailing_zeros()
    }).sum()
}

fn convert_to_mask(byte: &u8) -> u64 {
  if byte > &96 {
    1u64 << byte - 96
  } else {
    1u64 << byte - 38
  }
}

// Conceptually it's fine. That said...
// 1. I think you can use 'bytes()' instead of 'as_bytes().iter()'
// 2. There's no need to define the variabkes outside the iterators, this is not C 98
// 3. Use 'fold(0, |acc, x| acc | convert_to_mask(x))' for all the places you're building the bitsets/masks, it's much eazier to not need to track outside variables
// 4. If you're on nightly, remove that '.collect().chunks(3)' and instead use '.array_chunks::<3>()' on the iterator. Even if youre not on nightly, get a way to remove that 'collect', should be pretty easy to reimplement 'array_chunks' if you make it only work with arrays of size 3