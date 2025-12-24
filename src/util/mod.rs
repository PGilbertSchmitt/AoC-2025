use md5::Context;
use std::ops::Add;

pub mod heap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pair(pub i64, pub i64);

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

fn factors(x: i64) -> Vec<i64> {
    let mut fs = Vec::with_capacity((x / 4) as usize);
    let mut highest_known_factor = x;

    for value in 2..x {
        if value >= highest_known_factor {
            break;
        }
        if x % value == 0 {
            fs.push(value);
            let other_factor = x / value;
            if other_factor != value {
                fs.push(other_factor);
                highest_known_factor = other_factor;
            } else {
                break;
            }
        }
    }

    fs.push(x);
    fs.sort();
    fs.into_iter().rev().collect()
}

pub fn lowest_common_factor(x: i64, y: i64) -> Option<i64> {
    if x == 0 {
        return Some(y);
    }
    if y == 0 {
        return Some(x);
    }
    if x == y {
        return Some(x);
    }
    let x_factors = factors(x);
    if x_factors.is_empty() {
        return None;
    }
    let y_factors = factors(y);
    if y_factors.is_empty() {
        return None;
    }

    x_factors.iter().find(|&xf| y_factors.contains(xf)).copied()
}

#[test]
fn lowest_common_factor_test() {
    assert_eq!(None, lowest_common_factor(5, 7));
    assert_eq!(Some(2), lowest_common_factor(4, 6));
    assert_eq!(Some(4), lowest_common_factor(4, 8));
    assert_eq!(Some(3), lowest_common_factor(3, 9));
    assert_eq!(Some(3), lowest_common_factor(6, 9));
}
