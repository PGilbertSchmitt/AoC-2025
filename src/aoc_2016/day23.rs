use crate::aoc_2016::day12::{execute, parse_instructions};

const SAMPLE: &str = "
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

const INPUT: &str = include_str!("./inputs/day23.txt");

#[test]
fn part_1() {
    assert_eq!(3, execute(&parse_instructions(SAMPLE), 7, 0, 0, 0));
    assert_eq!(13958, execute(&parse_instructions(INPUT), 7, 0, 0, 0));
}

// This test takes nearly 5 seconds in release mode
// #[test]
// fn part_2() {
//     assert_eq!(479010518, execute(&parse_instructions(INPUT), 12, 0, 0, 0));
// }
