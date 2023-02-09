pub mod math;
pub mod sequences;
pub mod vecs;

pub fn digits<T>(n: T) -> Vec<u32>
where
    T: std::fmt::Display,
{
    n.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_test() {
        assert_eq!(digits(1234), vec![1, 2, 3, 4]);
        assert_eq!(digits(01234), vec![1, 2, 3, 4]);
        assert_eq!(digits(12340), vec![1, 2, 3, 4, 0]);
    }
}
