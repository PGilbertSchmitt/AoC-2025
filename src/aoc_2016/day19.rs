const SAMPLE: usize = 5;
const INPUT: usize = 3004953;

fn last_elf_part_1(input: usize) -> usize {
    let mut largest_power_of_two = 1;
    let mut cur_value = input;
    while cur_value > 1 {
        cur_value >>= 1;
        largest_power_of_two <<= 1;
    }
    let remainder = input - largest_power_of_two;
    2 * remainder + 1
}

// I used these two functions to find the solutions for 1-100 the slow way, which let me see that
// pattern that could help me write the fast, simple equation needed for the general example.

// fn generate(i: usize, list: Vec<usize>) -> (usize, Vec<usize>) {
//     let len = list.len();
//     let target_idx = (i + len / 2) % len;
//     let (start, end) = list.split_at(target_idx);
//     (target_idx, start.iter().chain(end[1..].iter()).cloned().collect())
// }

// fn last_elf_part_2_naive(input: usize) -> usize {
//     let mut elves: Vec<usize> = (1..input + 1).collect();
//     let mut i = 0;
//     while elves.len() > 1 {
//         let (target_idx, new_elves) = generate(i, elves);
//         elves = new_elves;
//         if target_idx > i {
//             i += 1;
//         }
//         if i >= elves.len() {
//             i = 0;
//         }
//     }
//     *elves.get(0).unwrap()
// }

fn last_elf_part_2(input: usize) -> usize {
    let mut largest_power_of_three = 1;
    while largest_power_of_three * 3 < input {
        largest_power_of_three *= 3;
    }

    let base = input - largest_power_of_three;
    if base <= largest_power_of_three {
        base
    } else {
        let remainder = base - largest_power_of_three;
        base + remainder
    }
}

#[test]
fn part_1() {
    assert_eq!(3, last_elf_part_1(SAMPLE));
    assert_eq!(1815603, last_elf_part_1(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(2, last_elf_part_2(SAMPLE));
    assert_eq!(1410630, last_elf_part_2(INPUT));
}
