use crate::utils::math::{is_even, DefaultMathType};

/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
#[derive(Debug)]
pub struct CollatzIterator {
    i: DefaultMathType,
    done: bool,
    started: bool,
}

impl CollatzIterator {
    pub fn new(i: DefaultMathType) -> Self {
        Self {
            i,
            done: false,
            started: false,
        }
    }
}

impl Iterator for CollatzIterator {
    type Item = DefaultMathType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if !self.started {
            self.started = true;
            return Some(self.i);
        }

        if is_even(self.i) {
            self.i /= 2;
        } else {
            self.i = self.i * 3 + 1;
        }

        if self.i == 1 {
            self.done = true;
            return Some(self.i);
        }
        Some(self.i)
    }
}
