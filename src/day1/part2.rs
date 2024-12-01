use hashbrown::HashMap;
use std::hash::Hash;

#[aoc(day1, part2)]
pub fn solution(input: &str) -> u32 {
    let right_freq: HashMap<&u32, usize> = HashMap::with_capacity(1000);
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|x| unsafe { x.split_once("   ").unwrap_unchecked() })
        .map(|line| unsafe {
            (
                u32::from_str_radix(line.0, 10).unwrap_unchecked(),
                u32::from_str_radix(line.1, 10).unwrap_unchecked(),
            )
        })
        .unzip();
    let right_freq = right.iter().my_counts();

    let sum = left.iter().fold(0, |acc, item| {
        acc + *item * *right_freq.get(item).unwrap_or(&0) as u32
    });
    sum
}

trait CustomIter: Iterator {
    #[inline(always)]
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
