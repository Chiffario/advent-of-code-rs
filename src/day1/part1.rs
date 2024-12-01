use itertools::Itertools;

#[aoc(day1, part1)]
pub fn solution(input: &str) -> u32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .into_iter()
        .flat_map(|x| x.split_once("   "))
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unzip();
    left.sort_unstable();
    right.sort_unstable();

    let common = left
        .into_iter()
        .interleave(right.into_iter())
        .tuples::<(_, _)>()
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum::<u32>();
    common
}
