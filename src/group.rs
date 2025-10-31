use crate::inv::Inv;
use std::{collections::HashSet, hash::Hash, ops::Mul};

pub trait Group: Default + Inv + Mul<Self, Output = Self> + Eq {
    fn has_subgroup(els: &HashSet<Self>) -> bool
    where
        Self: Hash + Clone,
    {
        if !els.contains(&Self::default()) {
            return false;
        }

        for a in els {
            for b in els {
                if !els.contains(&(a.clone() * b.clone())) {
                    return false;
                }
            }
        }

        true
    }
}

impl<T: Default + Inv + Mul<Self, Output = Self> + Eq> Group for T {}
