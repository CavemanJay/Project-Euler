use itertools::iproduct;

use crate::utils::math::is_palindrome;

fn solution(multiplier: u32) -> u32 {
    let range = (1 * multiplier..=9 * multiplier + multiplier - 1).rev();
    // let mut results = Vec::with_capacity((9 * multiplier + multiplier * 2) as usize);
    let indices = iproduct!(range.clone(), range.clone());
    indices
        .map(|(i, j)| i * j)
        .filter(|&n| is_palindrome(n))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 9009)]
    #[case(100, 906609)]
    fn solution_test(#[case] multiplier: u32, #[case] expected: u32) {
        assert_eq!(solution(multiplier), expected)
    }
}
