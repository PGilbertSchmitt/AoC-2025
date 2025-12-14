use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Pair(pub isize, pub isize);

impl Add<Pair> for Pair {
    type Output = Self;
    fn add(self, rhs: Pair) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
