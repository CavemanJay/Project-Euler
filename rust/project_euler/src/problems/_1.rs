use crate::utils::math;

fn solution(n: u32) -> u32 {
    (1..n)
        .filter(|n| math::is_multiple_of(*n, 3) || math::is_multiple_of(*n, 5))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 23)]
    #[case(1000, 233168)]
    fn solution_test(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(solution(input), expected)
    }
}
