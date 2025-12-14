// Didn't have to read far to see I was going to implement Conway's Game of Life
// Rest in Piece, hero

use std::collections::HashMap;

use crate::util::Pair;

type BoardState = [[bool; 100]; 100];

impl Pair {
    // They won't all fit on the board, but we can just ignore the
    // out-of-bounds cells tha are calculated
    pub fn neighbors(self) -> [Pair; 8] {
        let Pair(x, y) = self;
        [
            Pair(x + 1, y - 1),
            Pair(x + 1, y),
            Pair(x + 1, y + 1),
            Pair(x, y + 1),
            Pair(x - 1, y + 1),
            Pair(x - 1, y),
            Pair(x - 1, y - 1),
            Pair(x, y - 1),
        ]
    }
}

fn parse_config(input: &str) -> BoardState {
    let mut b: BoardState = [[false; 100]; 100];
    // let mut cur_line = Vec::with_capacity(100);
    let mut line_num = 0;
    let mut col_num = 0;

    for char in input.trim().chars() {
        match char {
            '.' => b[line_num][col_num] = false,
            '#' => b[line_num][col_num] = true,
            '\n' => {
                col_num = 0;
                line_num += 1;
                continue;
            }
            _ => unreachable!(),
        }
        col_num += 1;
    }

    b
}

fn next_state(b: BoardState, broken: bool) -> BoardState {
    let mut neighbor_map = HashMap::<Pair, u8>::new();

    for (x, line) in b.iter().enumerate() {
        for (y, cell) in line.iter().enumerate() {
            if *cell {
                let p = Pair(x as isize, y as isize);
                for neighbor in p.neighbors() {
                    neighbor_map
                        .entry(neighbor)
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }
            }
        }
    }

    let mut next_board = [[false; 100]; 100];

    for (x, line) in b.into_iter().enumerate() {
        for (y, cell) in line.into_iter().enumerate() {
            let key = Pair(x as isize, y as isize);
            let n_count = *neighbor_map.get(&key).unwrap_or(&0);
            if cell {
                next_board[x][y] = n_count == 2 || n_count == 3;
            } else {
                next_board[x][y] = n_count == 3;
            }
        }
    }

    if broken {
        next_board[0][0] = true;
        next_board[0][99] = true;
        next_board[99][0] = true;
        next_board[99][99] = true;
    }

    next_board
}

const INPUT: &str = include_str!("./inputs/day18.txt");

fn get_total_lights(broken: bool) -> usize {
    let mut b = parse_config(INPUT);
    for _ in 0..100 {
        b = next_state(b, broken);
    }

    b.into_iter().fold(0, |outer_sum, line| {
        line.into_iter()
            .fold(outer_sum, |sum, v| if v { sum + 1 } else { sum })
    })
}

#[test]
fn solutions() {
    assert_eq!(1061, get_total_lights(false));
    assert_eq!(1006, get_total_lights(true));
}
