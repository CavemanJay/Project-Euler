use crate::utils::{
    math::{self, DefaultMathType},
    sequences::fibonacci::FibIterator,
};

fn solution() -> DefaultMathType {
    FibIterator::new(1, 2)
        .take_while(|n| *n < 4_000_000)
        .filter(|n| math::is_even(*n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_test() {
        assert_eq!(solution(), 4613732)
    }
}
