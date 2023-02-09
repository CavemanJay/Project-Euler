use crate::utils::{digits, sequences::factorial::nth_factorial};

type Output = u32;
type Input = usize;

fn solution(n: Input) -> Output {
    digits(nth_factorial(n)).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 27)]
    #[case(100, 648)]
    fn solution_test(#[case] input: Input, #[case] expected: Output) {
        assert_eq!(solution(input), expected)
    }
}
