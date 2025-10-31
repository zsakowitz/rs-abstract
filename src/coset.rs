use std::{hash::Hash, ops::Mul};

pub fn lcoset<T: Mul<Output = T> + Clone + Hash + Eq>(lhs: &[T], rhs: T) -> Vec<T> {
    lhs.iter().cloned().map(|x| x * rhs.clone()).collect()
}

pub fn rcoset<T: Mul<Output = T> + Clone + Hash + Eq>(lhs: T, rhs: &[T]) -> Vec<T> {
    rhs.iter().cloned().map(|x| lhs.clone() * x).collect()
}
