#[aoc(day1, part2)]
pub fn run(input: &str) -> usize {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    input
        .lines()
        .map(|line| unsafe {
            (
                usize::from_str_radix(&line[0..5], 10).unwrap_unchecked(),
                usize::from_str_radix(&line[8..13], 10).unwrap_unchecked(),
            )
        })
        .for_each(|(lhs, rhs)| {
            left.push(lhs);
            right.push(rhs);
        });
    let mut right_freq: [usize; 100000] = [0; 100000];
    right.into_iter().for_each(|item| right_freq[item] += 1);

    left.iter()
        .fold(0, |acc: usize, item| acc + *item * right_freq[*item])
}
