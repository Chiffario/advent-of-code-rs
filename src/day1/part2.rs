use hashbrown::HashMap;
use std::hash::Hash;

#[aoc(day1, part2)]
pub fn solution(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .into_iter()
        .flat_map(|x| x.split_once("   "))
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .unzip();
    left.sort_unstable();
    right.sort_unstable();

    let left_freq = left.iter().my_counts();
    let right_freq = right.iter().my_counts();

    let sum = left_freq.iter().fold(0, |acc, (item, count)| {
        acc + *item * *count as u32 * *right_freq.get(item).unwrap_or(&1) as u32
    });
    sum
}

trait CustomIter: Iterator {
    fn my_counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
}

impl<T: ?Sized> CustomIter for T where T: Iterator {}
