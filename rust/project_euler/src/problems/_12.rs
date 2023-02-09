use crate::utils::{math::factors::factors, sequences::triangles::TriangleIterator};

type Output = usize;
type Input = usize;

fn solution(n: Input) -> Output {
    let triangles = TriangleIterator::new();
    let (solution, _factors) = triangles
        .map(|n| (n, factors(n)))
        .find(|(_, factors)| factors.len() > n)
        .unwrap();
    solution as Output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(5, 28)]
    #[case(500, 76576500)]
    fn solution_test(#[case] input: Input, #[case] expected: Output) {
        assert_eq!(solution(input), expected)
    }
}
