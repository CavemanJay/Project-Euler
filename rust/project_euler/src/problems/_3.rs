use crate::utils::math::{self, DefaultMathType};
use crate::utils::sequences;

fn solution() -> DefaultMathType {
    let n = 600851475143;
    // *math::prime_factors(n).iter().max().unwrap()
    let factors = math::factors(n);
    dbg!(factors.last());
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::utils::math::prime_factors;
    use rstest::rstest;

    use super::*;
    #[test]
    fn solution_test() {
        assert_eq!(solution(), 0)
    }

    #[test]
    fn largest_prime_factor_test() {
        assert_eq!(*prime_factors(13195).iter().max().unwrap(), 29)
    }

    #[rstest]
    #[case(13195, vec![5,7,13,29])]
    fn prime_factors_test(#[case] input: i64, #[case] expected: Vec<DefaultMathType>) {
        assert_eq!(prime_factors(input), expected)
    }
}
