use itertools::Itertools;
use num::{integer::Roots, Zero};
use std::ops::Rem;

pub type DefaultMathType = i64;

#[inline]
pub fn is_multiple_of<T>(n: T, x: T) -> bool
where
    <T as Rem>::Output: PartialEq<T>,
    T: std::ops::Rem + num::Zero,
{
    n % x == Zero::zero()
}

#[inline]
pub const fn is_even(n: DefaultMathType) -> bool {
    n % (2 as DefaultMathType) == 0
}

pub fn is_prime(n: DefaultMathType) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

pub mod factors {
    use std::collections::HashSet;

    use itertools::Itertools;
    use num::integer::Roots;

    use super::{is_multiple_of, is_prime, DefaultMathType};

    /// Taken from: <https://gist.github.com/qolop/71ef78c394db822756d58cac9993db77>
    pub fn factors(n: DefaultMathType) -> HashSet<DefaultMathType> {
        let step = if n % 2 == 1 { 2 } else { 1 };
        (1..=n.sqrt())
            .step_by(step)
            .filter(|&i| n % i == 0)
            .map(|i| [i, n / i])
            .flatten()
            .collect::<HashSet<DefaultMathType>>()
    }

    pub fn proper_factors(n: DefaultMathType) -> HashSet<DefaultMathType> {
        let step = if n % 2 == 1 { 2 } else { 1 };
        (1..=n.sqrt())
            .step_by(step)
            .filter(|&i| n % i == 0)
            .map(|i| [i, n / i])
            .flatten()
            .sorted()
            .dropping_back(1)
            .collect::<HashSet<DefaultMathType>>()
    }

    pub fn prime_factors(n: DefaultMathType) -> HashSet<DefaultMathType> {
        factors(n)
            .iter()
            .filter(|&&n| is_prime(n))
            .map(ToOwned::to_owned)
            .collect()
    }

    pub struct FactorsIterator {
        i: DefaultMathType,
        n: DefaultMathType,
    }

    impl FactorsIterator {
        pub fn new(n: DefaultMathType) -> Self {
            Self { i: 0, n }
        }
    }

    impl Iterator for FactorsIterator {
        type Item = DefaultMathType;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                self.i += 1;
                if self.i == self.n {
                    return None;
                }
                if is_multiple_of(self.n, self.i) {
                    return Some(self.i);
                }
            }
        }
    }
}

pub fn is_palindrome<T>(n: T) -> bool
where
    // String: From<T>,
    T: ToString,
{
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}
