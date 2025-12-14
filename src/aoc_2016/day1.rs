use fxhash::FxHashSet;
use logos::{Lexer, Logos};

fn parse_dist(lex: &mut Lexer<Token>) -> i64 {
    lex.slice()[1..].parse::<i64>().unwrap()
}

#[derive(Logos)]
#[logos(skip r"[ ,\n]")]
enum Token {
    #[regex("R[0-9]+", parse_dist)]
    Right(i64),

    #[regex("L[0-9]+", parse_dist)]
    Left(i64),
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn vector(&self, token: &Token) -> (i64, i64, Self) {
        match token {
            &Token::Right(x) => match self {
                Self::Up => (x, 0, Self::Right),
                Self::Right => (0, -x, Self::Down),
                Self::Down => (-x, 0, Self::Left),
                Self::Left => (0, x, Self::Up),
            },
            &Token::Left(x) => match self {
                Self::Up => (-x, 0, Self::Left),
                Self::Left => (0, -x, Self::Down),
                Self::Down => (x, 0, Self::Right),
                Self::Right => (0, x, Self::Up),
            },
        }
    }
}

fn dist_away(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;

    let tokens = Token::lexer(input);
    for token in tokens.into_iter() {
        let token = token.unwrap();
        let (new_x, new_y, new_dir) = dir.vector(&token);
        x += new_x;
        y += new_y;
        dir = new_dir;
    }

    return x.abs() + y.abs();
}

// Removing `.rev()` doesn't make a difference, but I think it's technically correct because it could matter in theory
fn points_in_vec(
    (start_x, start_y): (i64, i64),
    (end_x, end_y): (i64, i64),
    facing: &Dir,
) -> Vec<(i64, i64)> {
    match facing {
        Dir::Up => (start_y + 1..end_y + 1).map(|y| (start_x, y)).collect(),
        Dir::Down => (end_y..start_y).rev().map(|y| (start_x, y)).collect(),
        Dir::Right => (start_x + 1..end_x + 1).map(|x| (x, start_y)).collect(),
        Dir::Left => (end_x..start_x).rev().map(|x| (x, start_y)).collect(),
    }
}

fn first_visited_twice(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::Up;

    let mut visited = FxHashSet::<(i64, i64)>::default();

    let tokens = Token::lexer(input);
    for token in tokens.into_iter() {
        let token = token.unwrap();
        let (delta_x, delta_y, new_dir) = dir.vector(&token);
        let new_x = x + delta_x;
        let new_y = y + delta_y;
        for coord in points_in_vec((x, y), (new_x, new_y), &new_dir) {
            if visited.contains(&coord) {
                let (final_x, final_y) = coord;
                return final_x.abs() + final_y.abs();
            } else {
                visited.insert(coord);
            }
        }

        x = new_x;
        y = new_y;
        dir = new_dir;
    }

    unreachable!()
}

const SAMPLE_1: &'static str = "R5, L5, R5, R3";
const SAMPLE_2: &'static str = "R8, R4, R4, R8";
const INPUT: &'static str = include_str!("./inputs/day1.txt");

#[test]
fn part1() {
    assert_eq!(12, dist_away(SAMPLE_1));
    assert_eq!(291, dist_away(INPUT));
}

#[test]
fn part2() {
    assert_eq!(4, first_visited_twice(SAMPLE_2));
    assert_eq!(159, first_visited_twice(INPUT));
}
