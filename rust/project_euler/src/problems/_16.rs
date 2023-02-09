use num::BigUint;

type Output = u32;
type Input = u32;

fn solution(n: Input) -> Output {
    BigUint::from(2u8)
        .pow(n)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(15, 26)]
    #[case(1_000, 1366)]
    fn solution_test(#[case] input: Input, #[case] expected: Output) {
        assert_eq!(solution(input), expected)
    }
}
