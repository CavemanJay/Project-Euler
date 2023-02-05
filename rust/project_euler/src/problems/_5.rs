use crate::utils::math;

fn solution(end: usize) -> u32 {
    let y = (1..)
        .filter(|&i| (1..=end).all(|n| math::is_multiple_of(i, n)))
        .next()
        .unwrap();
    // .take(1)
    // .collect::<Vec<_>>();
    y as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 2520)]
    #[case(20, 232792560)]
    fn solution_test(#[case] input: usize, #[case] expected: u32) {
        assert_eq!(solution(input), expected)
    }
}
