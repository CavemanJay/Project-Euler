// 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,

use num::{bigint::ToBigInt, BigInt, One};

pub fn nth_factorial(n: usize) -> BigInt {
    (1..=n).fold(BigInt::one(), |acc, x| {
        acc.checked_mul(&x.to_bigint().unwrap()).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_factorial_test() {
        assert_eq!(nth_factorial(10), 3628800.to_bigint().unwrap())
    }
}
