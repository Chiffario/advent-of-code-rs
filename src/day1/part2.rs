// #[aoc(day1, part2)]
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
    let mut right_freq: [usize; 100000] = [0; 100000];
    right.into_iter().for_each(|item| right_freq[item] += 1);

    left.iter()
        .fold(0, |acc: usize, item| acc + *item * right_freq[*item])
}

unsafe fn parse_digit_unrolled(s: &str) -> usize {
    (*s.as_bytes().get_unchecked(0) as usize - b'0' as usize) * 10000
        + (*s.as_bytes().get_unchecked(1) as usize - b'0' as usize) * 1000
        + (*s.as_bytes().get_unchecked(2) as usize - b'0' as usize) * 100
        + (*s.as_bytes().get_unchecked(3) as usize - b'0' as usize) * 10
        + (*s.as_bytes().get_unchecked(4) as usize - b'0' as usize) * 1
}
