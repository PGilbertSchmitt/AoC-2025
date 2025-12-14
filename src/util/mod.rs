use std::ops::Add;
use md5::Context;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Pair(pub isize, pub isize);

impl Add<Pair> for Pair {
    type Output = Self;
    fn add(self, rhs: Pair) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn generate_hash(input: &[u8]) -> [u8; 16] {
    let mut ctx = Context::new();
    ctx.consume(input);
    ctx.finalize().0
}

pub fn generate_padded_hash(base: &[u8], padding: usize) -> [u8; 16] {
    let mut ctx = Context::new();
    ctx.consume(base);
    ctx.consume(padding.to_string().as_bytes());
    ctx.finalize().0
}
