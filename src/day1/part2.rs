use itertools::Interleave;
use itertools::Itertools;
use std::fmt::Display;

#[aoc(day1, part2)]
pub fn solution(input: &str) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .into_iter()
        .flat_map(|x| x.split_once("   "))
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unzip();
    left.sort_unstable();
    right.sort_unstable();

    let left_freq = left.clone().into_iter().counts();
    let right_freq = right.clone().into_iter().counts();

    let mut sum = 0;
    for (el, count) in left_freq {
        let a = el * (count as i32) * *right_freq.get(&el).unwrap_or(&0) as i32;
        sum += a;
    }

    sum
}
