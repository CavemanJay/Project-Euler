use primes::PrimeSet;

type Output = u64;
type Input = u64;

fn solution(n: Input) -> Output {
    let mut pset = primes::Sieve::new();
    let primes = pset.iter();
    primes.take_while(|&p| p < n).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 17)]
    #[case(2_000_000, 142913828922)]
    fn solution_test(#[case] input: Input, #[case] expected: Output) {
        assert_eq!(solution(input), expected)
    }
}
