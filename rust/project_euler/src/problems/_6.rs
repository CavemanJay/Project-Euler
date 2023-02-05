fn solution(n: u32) -> u32 {
    let sum_of_squares: u32 = (1..=n).map(|i| i.pow(2)).sum();
    let square_of_sums = (1..=n).sum::<u32>().pow(2);
    square_of_sums - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 2640)]
    #[case(100, 25164150)]
    fn solution_test(#[case] input: u32, #[case] expected: u32) {
        assert_eq!(solution(input), expected)
    }
}
