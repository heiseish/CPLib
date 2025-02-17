use std::collections::*;

pub type Stack<T> = Vec<T>;
pub type MinimumStack<T> = Stack<(T, T)>; // 2-nd element is the minimum in the current stack
