use itertools::Itertools;

#[aoc(day1, part1)]
pub fn solution(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|x| unsafe { x.split_once("   ").unwrap_unchecked() })
        .map(|line| unsafe {
            (
                u32::from_str_radix(line.0, 10).unwrap_unchecked(),
                u32::from_str_radix(line.1, 10).unwrap_unchecked(),
            )
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();

    let common = left
        .into_iter()
        .interleave(right.into_iter())
        .tuples::<(_, _)>()
        .fold(0, |acc, (lhs, rhs)| acc + lhs.abs_diff(rhs));
    common
}
