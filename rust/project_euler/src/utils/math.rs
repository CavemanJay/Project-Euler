use num::{Zero, integer::sqrt};
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
pub fn is_even(n: DefaultMathType) -> bool {
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
pub fn factors_functional(n: DefaultMathType) -> Vec<DefaultMathType> {
    (1..n + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<DefaultMathType>>()
}

/// Taken from: <https://gist.github.com/qolop/71ef78c394db822756d58cac9993db77>
pub fn factors(n: DefaultMathType) -> Vec<DefaultMathType> {
    let step = if n % 2 == 1 { 2 } else { 1 };
    let range = (1..sqrt(n)+1).step_by(step);
}

pub fn prime_factors(n: DefaultMathType) -> Vec<DefaultMathType> {
    factors(n)
        .iter()
        .filter(|n| is_prime(**n))
        .map(ToOwned::to_owned)
        .collect()
}
