fn solution(n: u64) -> u64 {
    *primes::factors_uniq(n).iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::math::prime_factors;
    use rstest::rstest;

    use super::*;
    #[rstest]
    #[case(13195, 29)]
    #[case(600851475143, 6857)]
    fn solution_test(#[case] input: u64, #[case] expected: u64) {
        assert_eq!(solution(input), expected)
    }

    #[test]
    fn largest_prime_factor_test() {
        assert_eq!(*prime_factors(13195).iter().max().unwrap(), 29)
    }

    #[rstest]
    #[case(13195, vec![5,7,13,29])]
    fn prime_factors_test(
        #[case] input: i64,
        #[case] expected: Vec<crate::utils::math::DefaultMathType>,
    ) {
        assert_eq!(prime_factors(input), expected)
    }
}
