use primes::PrimeSet;

fn solution(n: usize) -> u64 {
    let mut pset = primes::Sieve::new();
    pset.iter().nth(n - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(6, 13)]
    #[case(10_001, 104743)]
    fn solution_test(#[case] input: usize, #[case] expected: u64) {
        assert_eq!(solution(input), expected)
    }
}
