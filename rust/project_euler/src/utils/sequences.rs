use super::math::DefaultMathType;

/// Taken from: <https://www.umcconnell.net/posts/2021-03-13-fibonacci-rust/#1-standard>
pub fn fib(n: DefaultMathType) -> DefaultMathType {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
    }

    b
}

pub struct FibIterator {
    a: DefaultMathType,
    b: DefaultMathType,
}

impl FibIterator {
    pub fn new(a: DefaultMathType, b: DefaultMathType) -> Self {
        Self { a, b }
    }
}

impl Default for FibIterator {
    fn default() -> Self {
        FibIterator { a: 1, b: 1 }
    }
}

impl Iterator for FibIterator {
    type Item = DefaultMathType;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.a;
        self.a = self.b;
        self.b = curr + self.a;

        Some(curr)
    }
}
