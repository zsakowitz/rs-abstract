use crate::{
    inv::Inv,
    latex::ToLatex,
    supsub::{SubS, SupS},
};
use std::{
    fmt::{Debug, Display},
    ops::Mul,
};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct D<const N: u16> {
    rev: bool,
    idx: u16,
}

impl<const N: u16> D<N> {
    const fn new(rev: bool, idx: u16) -> Self {
        assert!(N != 0);
        Self { rev, idx: idx % N }
    }

    pub const fn rot(idx: u16) -> Self {
        Self::new(false, idx)
    }

    pub const fn flip(idx: u16) -> Self {
        Self::new(true, idx)
    }
}

impl<const N: u16> Inv for D<N> {
    fn inv(self) -> Self {
        if self.rev {
            self
        } else {
            Self {
                rev: false,
                idx: N - self.idx,
            }
        }
    }
}

impl<const N: u16> Mul for D<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (rev, idx) = match (self.rev, self.idx, rhs.rev, rhs.idx) {
            (false, a, false, b) => (false, (a + b) % N),
            (false, a, true, b) => (true, (a + b) % N),
            (true, a, false, b) => (true, (N + a - b) % N),
            (true, a, true, b) => (false, (N + a - b) % N),
        };
        Self { rev, idx }
    }
}

impl<const N: u16> Display for D<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.rev, self.idx) {
            (false, 0) => write!(f, "e"),
            (false, a) => write!(f, "r{}", SupS(a)),
            (true, 0) => write!(f, "M{}", SubS(N)),
            (true, a) => write!(f, "M{}", SubS(a)),
        }
    }
}

impl<const N: u16> ToLatex for D<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.rev, self.idx) {
            (false, 0) => write!(f, "e"),
            (false, a) => write!(f, "r_{{{}}}", a),
            (true, 0) => write!(f, "M_{{{}}}", N),
            (true, a) => write!(f, "M_{{{}}}", a),
        }
    }
}

impl<const N: u16> Debug for D<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}
