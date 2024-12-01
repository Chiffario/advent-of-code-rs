use std::iter::zip;

#[aoc(day1, part1)]
pub fn run(input: &str) -> usize {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    input
        .split('\n')
        .map(|line| unsafe {
            (
                parse_digit_unrolled(&line[0..5]),
                parse_digit_unrolled(&line[8..13]),
            )
        })
        .for_each(|(lhs, rhs)| {
            left.push(lhs);
            right.push(rhs);
        });
    left.sort_unstable();
    right.sort_unstable();

    zip(left, right).fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs))
}
unsafe fn parse_digit_unrolled(s: &str) -> usize {
    (*s.as_bytes().get_unchecked(0) as usize - b'0' as usize) * 10000
        + (*s.as_bytes().get_unchecked(1) as usize - b'0' as usize) * 1000
        + (*s.as_bytes().get_unchecked(2) as usize - b'0' as usize) * 100
        + (*s.as_bytes().get_unchecked(3) as usize - b'0' as usize) * 10
        + (*s.as_bytes().get_unchecked(4) as usize - b'0' as usize) * 1
}
