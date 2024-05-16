use std::cmp;
use std::collections::*;

pub type Stack<T> = Vec<T>;
pub type MinimumStack<T> = Stack<(T, T)>; // 2-nd element is the minimum in the current stack

#[derive(Default)]
pub struct MinimumQueue<T>
where
    T: Copy + Ord,
{
    s1: MinimumStack<T>,
    s2: MinimumStack<T>,
}

impl<T> MinimumQueue<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn minimum(&self) -> T {
        match (self.s1.as_slice().last(), self.s2.as_slice().last()) {
            (None, None) => panic!(),
            (Some(&x), None) => x.1,
            (None, Some(&x)) => x.1,
            (Some(&x), Some(&y)) => cmp::min(x.1, y.1),
        }
    }

    pub fn push(&mut self, val: T) {
        let minimum = {
            match self.s1.as_slice().last() {
                None => val,
                Some(&x) => cmp::min(val, x1),
            }
        };
        self.s2.push((val, minimum));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.s2.is_empty() {
            if self.s1.is_empty() {
                return None;
            }
            while !self.s1.is_empty() {
                let val = s1.pop().unwrap();
                let minimum = {
                    match self.s2.as_slice().last() {
                        None => val,
                        Some(&x) => cmp::min(val, x1),
                    }
                };
                self.s2.push((element, minimum));
            }
        }
        self.s2.pop()
    }
}
