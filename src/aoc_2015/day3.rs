use std::collections::HashSet;

use crate::util::Pair;

fn dir_diff(dir: u8) -> Pair {
    match dir {
        b'^' => Pair(0, 1),
        b'v' => Pair(0, -1),
        b'>' => Pair(1, 0),
        b'<' => Pair(-1, 0),
        _ => unreachable!(),
    }
}

fn houses_visited(input: &[u8]) -> usize {
    let start = Pair(0, 0);
    let mut houses: HashSet<Pair> = HashSet::new();
    houses.insert(start.clone());

    input
        .iter()
        .fold((houses, start), |(mut house_set, last_house), dir| {
            let next_house = last_house + dir_diff(*dir);
            house_set.insert(next_house.clone());
            (house_set, next_house)
        })
        .0
        .len()
}

fn houses_visited_with_robo_santa(input: &[u8]) -> usize {
    let santa_start: Pair = Pair(0, 0);
    let robot_start: Pair = Pair(0, 0);
    let mut houses: HashSet<Pair> = HashSet::new();
    houses.insert(santa_start.clone());

    input
        .iter()
        .enumerate()
        .fold((houses, santa_start, robot_start), |acc, (index, dir)| {
            let (mut house_set, last_santa, last_robot) = acc;
            let diff = dir_diff(*dir);
            let (next_santa, next_robot, new_pos) = if index % 2 == 0 {
                // Real Santa's turn
                let new_pos = last_santa + diff;
                (new_pos.clone(), last_robot, new_pos)
            } else {
                // Robo-Santa's turn
                let new_pos = last_robot + diff;
                (last_santa, new_pos.clone(), new_pos)
            };
            house_set.insert(new_pos);
            (house_set, next_santa, next_robot)
        })
        .0
        .len()
}

const S1: &[u8] = b"^>v<";
const S2: &[u8] = b"^v^v^v^v^v";
const INPUT: &[u8] = include_bytes!("./inputs/day3.txt");

#[test]
fn part_1() {
    assert_eq!(4, houses_visited(S1));
    assert_eq!(2, houses_visited(S2));
    assert_eq!(2565, houses_visited(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(3, houses_visited_with_robo_santa(S1));
    assert_eq!(11, houses_visited_with_robo_santa(S2));
    assert_eq!(2639, houses_visited_with_robo_santa(INPUT));
}
