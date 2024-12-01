use std::{iter::zip};

#[aoc(day1, part1)]
pub fn run(input: &str) -> usize {
    let mut left = [0; 1000];
    let mut right = [0; 1000];
    input
        .split('\n')
        .map(|line| unsafe { parse_two_numbers_unrolled(line) })
        .enumerate()
        .for_each(|(idx, (lhs, rhs))| {
            left[idx] = lhs;
            right[idx] = rhs;
        });
    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs))
}
#[inline(always)]
unsafe fn parse_two_numbers_unrolled(s: &str) -> (usize, usize) {
   (
        // Left number
        (*s.as_bytes().get_unchecked(0) as usize - b'0' as usize) * 10000
            + (*s.as_bytes().get_unchecked(1) as usize - b'0' as usize) * 1000
            + (*s.as_bytes().get_unchecked(2) as usize - b'0' as usize) * 100
            + (*s.as_bytes().get_unchecked(3) as usize - b'0' as usize) * 10
            + (*s.as_bytes().get_unchecked(4) as usize - b'0' as usize) * 1,
        // Right number
        (*s.as_bytes().get_unchecked(8) as usize - b'0' as usize) * 10000
            + (*s.as_bytes().get_unchecked(9) as usize - b'0' as usize) * 1000
            + (*s.as_bytes().get_unchecked(10) as usize - b'0' as usize) * 100
            + (*s.as_bytes().get_unchecked(11) as usize - b'0' as usize) * 10
            + (*s.as_bytes().get_unchecked(12) as usize - b'0' as usize) * 1,
    )
}
