use itertools::Itertools;

#[aoc(day1, part1)]
pub fn solution(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .into_iter()
        .flat_map(|x| x.split_once("   "))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
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
