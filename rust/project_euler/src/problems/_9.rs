use num::traits::Pow;

use crate::utils::math;

type Output = i32;
type Input = ();

fn solution() -> Output {
    for c in 1..i32::MAX {
        for b in 1..c {
            for a in 1..b {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1_000 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("Result not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_test() {
        assert_eq!(solution(), 31875000)
    }
}
