use crate::utils::math;

type Output = ();
type Input = usize;

fn solution(n: Input) -> Output {
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 23)]
    #[case(1000, 233168)]
    fn solution_test(#[case] input: Input, #[case] expected: Output) {
        assert_eq!(solution(input), expected)
    }
}
