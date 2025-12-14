use fxhash::FxHashSet;

use crate::util::generate_padded_hash;

fn from_useful_hash(hash: [u8; 16]) -> Option<char> {
    if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 {
        Some(format!("{:x}", hash[2]).chars().next().unwrap())
    } else {
        None
    }
}

fn from_inspired_hash(hash: [u8; 16]) -> Option<(usize, char)> {
    if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 {
        let position = hash[2] as usize;
        if position > 7 {
            return None;
        }

        let password_char = hash[3] >> 4;
        Some((
            position,
            format!("{:x}", password_char).chars().next().unwrap(),
        ))
    } else {
        None
    }
}

fn find_password(input: &[u8]) -> String {
    let mut i = 0;
    let mut password_chars: Vec<char> = Vec::new();
    while password_chars.len() < 8 {
        let hash = generate_padded_hash(input, i);
        if let Some(ch) = from_useful_hash(hash) {
            password_chars.push(ch);
        }

        i += 1;
    }
    password_chars.iter().collect()
}

fn find_inspired_password(input: &[u8]) -> String {
    let mut i = 0;
    let mut password_chars: [char; 8] = ['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut remaining_positions = FxHashSet::from_iter([0, 1, 2, 3, 4, 5, 6, 7].iter());
    while remaining_positions.len() > 0 {
        let hash = generate_padded_hash(input, i);
        if let Some((pos, ch)) = from_inspired_hash(hash) {
            if remaining_positions.contains(&pos) {
                password_chars[pos] = ch;
                remaining_positions.remove(&pos);
            }
        }

        i += 1;
    }
    password_chars.iter().collect()
}

const SAMPLE: &[u8] = b"abc";
const INPUT: &[u8] = b"ffykfhsq";

#[test]
fn part1() {
    assert_eq!("18f47a30", find_password(SAMPLE));
    assert_eq!("c6697b55", find_password(INPUT));
}

#[test]
fn part2() {
    assert_eq!("05ace8e3", find_inspired_password(SAMPLE));
    assert_eq!("8c35d1ab", find_inspired_password(INPUT));
}
