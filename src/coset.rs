use std::{hash::Hash, ops::Mul};

pub fn lcoset<T: Mul<Output = T> + Clone + Hash + Eq>(
    lhs: impl IntoIterator<Item = T>,
    rhs: T,
) -> Vec<T> {
    lhs.into_iter()
        .map(move |lhs| lhs.clone() * rhs.clone())
        .collect()
}

pub fn rcoset<T: Mul<Output = T> + Clone + Hash + Eq>(
    lhs: T,
    rhs: impl IntoIterator<Item = T>,
) -> Vec<T> {
    rhs.into_iter()
        .map(|rhs| lhs.clone() * rhs.clone())
        .collect()
}
