use num::Zero;
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

/// Taken from: <https://gist.github.com/qolop/71ef78c394db822756d58cac9993db77>
pub fn factors(n: DefaultMathType) -> Vec<DefaultMathType> {
    (1..n / 2 + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<DefaultMathType>>()
}

pub fn factors_rev(n: DefaultMathType) -> impl Iterator<Item = DefaultMathType> {
    (1..n / 2 + 1).rev().into_iter().filter(move |x| n % x == 0)
}

pub fn prime_factors(n: DefaultMathType) -> Vec<DefaultMathType> {
    factors(n)
        .iter()
        .filter(|n| is_prime(**n))
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

pub fn is_palindrome<T>(n: T) -> bool
where
    // String: From<T>,
    T: ToString,
{
    let s = n.to_string();
    s.chars().rev().collect::<String>() == s
}
