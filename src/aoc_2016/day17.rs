use std::collections::VecDeque;

use crate::util::generate_hash;

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn update_path(&self, path: &str) -> String {
        let ch = match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Up => 'U',
            Self::Down => 'D',
        };
        format!("{path}{ch}")
    }
}

struct State {
    col: u8,
    row: u8,
    path: String,
}

impl State {
    fn next_states(&self) -> Vec<Self> {
        let mut doors = Vec::new();
        let Self { col, row, path } = self;
        let col = *col;
        let row = *row;
        let hash = generate_hash(path.as_bytes());
        let byte_1 = hash[0];
        let byte_2 = hash[1];

        // The check for b-f is different depending on whether the hex char is the first or second
        // nibble. First nibble is simply if the byte is greater than 0xAF, whereas with the second
        // nibble, we remove the first nibble by ANDing it with 0xF then check if it's greater
        // than 0xA

        // Up is the first hex char
        if row > 0 && byte_1 > 0xAF {
            doors.push(Self {
                col,
                row: row - 1,
                path: Dir::Up.update_path(path),
            });
        }
        // Down is the second
        if row < 3 && (byte_1 & 0xF) > 0xA {
            doors.push(Self {
                col,
                row: row + 1,
                path: Dir::Down.update_path(path),
            });
        }
        // Left is the third
        if col > 0 && byte_2 > 0xAF {
            doors.push(Self {
                col: col - 1,
                row,
                path: Dir::Left.update_path(path),
            });
        }
        // Right is the fourth
        if col < 3 && (byte_2 & 0x0F) > 0xA {
            doors.push(Self {
                col: col + 1,
                row,
                path: Dir::Right.update_path(path),
            });
        }
        doors
    }

    fn done(&self) -> bool {
        self.col == 3 && self.row == 3
    }
}

const SAMPLE_1: &str = "ihgpwlah";
const SAMPLE_2: &str = "kglvqrro";
const SAMPLE_3: &str = "ulqzkmiv";
const INPUT: &str = "yjjvjgan";

fn paths_out(input: &str) -> (String, usize) {
    let mut paths = VecDeque::new();
    paths.push_back(State {
        col: 0,
        row: 0,
        path: String::from(input),
    });

    let mut shortest = None;
    let mut last_len = 0;
    while let Some(cur) = paths.pop_front() {
        for next in cur.next_states() {
            if next.done() {
                last_len = next.path.len() - input.len();
                if shortest.is_none() {
                    shortest.replace(String::from(&next.path[input.len()..]));
                }
            } else {
                paths.push_back(next);
            }
        }
    }

    (shortest.unwrap(), last_len)
}

#[test]
fn test() {
    let (shortest, longest_len) = paths_out(SAMPLE_1);
    assert_eq!("DDRRRD", &shortest);
    assert_eq!(370, longest_len);
    let (shortest, longest_len) = paths_out(SAMPLE_2);
    assert_eq!("DDUDRLRRUDRD", &shortest);
    assert_eq!(492, longest_len);
    let (shortest, longest_len) = paths_out(SAMPLE_3);
    assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", &shortest);
    assert_eq!(830, longest_len);
    let (shortest, longest_len) = paths_out(INPUT);
    assert_eq!("RLDRUDRDDR", &shortest);
    assert_eq!(498, longest_len);
}
