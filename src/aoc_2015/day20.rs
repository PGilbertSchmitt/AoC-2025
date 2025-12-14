use itertools::Itertools;

fn first_idx_over_value(elements: &Vec<usize>, value: usize) -> usize {
    elements
        .iter()
        .enumerate()
        .find_or_first(|(_, presents)| **presents >= value)
        .unwrap()
        .0
}

fn calculate_present(value: usize) -> (usize, usize) {
    let smaller_value = value / 10;
    let mut part1: Vec<usize> = vec![0; smaller_value];
    let mut part2: Vec<usize> = vec![0; smaller_value];

    for x in 1..smaller_value {
        let mut visits = 0;
        for y in (x..smaller_value).step_by(x) {
            part1[y] += x * 10;
            if visits < 50 {
                visits += 1;
                part2[y] += x * 11;
            }
        }
    }

    (
        first_idx_over_value(&part1, value),
        first_idx_over_value(&part2, value),
    )
}

#[test]
fn both_parts() {
    let (sample_part_1, sample_part_2) = calculate_present(150);
    assert_eq!(8, sample_part_1);
    assert_eq!(8, sample_part_2);
    let (input_part_1, input_part_2) = calculate_present(33100000);
    assert_eq!(776160, input_part_1);
    assert_eq!(786240, input_part_2);
}
