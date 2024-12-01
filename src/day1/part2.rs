use hashbrown::HashMap;
use std::hash::Hash;

#[aoc(day1, part2)]
pub fn solution(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    input
        .lines()
        .map(|line| unsafe {
            (
                u32::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
                u32::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
            )
        })
        .for_each(|(lhs, rhs)| {
            left.push(lhs);
            right.push(rhs);
        });
    let right_freq = right.iter().my_counts();

    left.iter().fold(0, |acc, item| {
        acc + *item * *right_freq.get(item).unwrap_or(&0) as u32
    })
}

trait CustomIter: Iterator {
    #[inline(always)]
    fn my_counts(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = HashMap::with_capacity(1000);
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
}

impl<T: ?Sized> CustomIter for T where T: Iterator {}
