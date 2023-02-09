use itertools::Itertools;

use crate::utils::{
    math::{self, DefaultMathType},
    sequences::collatz::CollatzIterator,
};

type Output = DefaultMathType;
type Input = usize;

fn solution() -> Output {
    (1..1_000_000)
        .map(|start| (start, CollatzIterator::new(start).count()))
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .last()
        .unwrap()
        .0
    // .collect_vec();
}

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[test]
    fn solution_test() {
        assert_eq!(solution(), 837799)
    }
}
