use itertools::Itertools;
use crate::utils::math::factors::proper_factors;

type Output = i64;

fn solution() -> Output {
    let d = |n| proper_factors(n).iter().sum::<Output>();
    (1..10_000)
        .filter_map(|a| {
            let b = d(a);
            let d_b = d(b);
            if a != b && d_b == a {
                Some([a, b])
            } else {
                None
            }
        })
        .flatten()
        .sorted()
        .dedup()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_test() {
        assert_eq!(solution(), 31626)
    }
}
