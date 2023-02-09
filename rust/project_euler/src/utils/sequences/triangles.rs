use crate::utils::math::DefaultMathType;

pub struct TriangleIterator {
    i: usize,
    current: usize,
}

impl TriangleIterator {
    pub fn new() -> Self {
        Self { i: 1, current: 0 }
    }
}

impl Iterator for TriangleIterator {
    type Item = DefaultMathType;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += self.i;
        self.i += 1;
        Some(self.current as Self::Item)
    }
}

pub fn nth_triangle_num(n: usize) -> DefaultMathType {
    (1..).take(n).fold(0, |x, y| x + y)
}
