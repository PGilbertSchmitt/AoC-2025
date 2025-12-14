use itertools::Itertools;

const SAMPLE: &'static str = "
1
2
3
4
5
7
8
9
10
11";

const INPUT: &'static str = include_str!("./inputs/day24.txt");

fn parse_packages(input: &str) -> Vec<usize> {
    let mut packages: Vec<usize> = input
        .trim()
        .split("\n")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    packages.sort();
    packages.into_iter().rev().collect()
}

fn find_lowest_config(packages: &Vec<usize>, compartments: usize) -> usize {
    let total_size: usize = packages.iter().sum();
    if total_size % compartments != 0 {
        panic!("Packages cannot be split into {compartments} equal groups");
    }
    let compartment_weight = total_size / compartments;

    let mut len_of_smallest_valid_set = usize::MAX;
    let mut smallest_valid_quantum_entaglement = usize::MAX;
    for subset in packages.iter().powerset() {
        // The powersets start short and get longer, which lets us stop early
        if subset.len() > len_of_smallest_valid_set {
            break;
        }
        if compartment_weight == subset.iter().copied().sum() {
            if subset.len() <= len_of_smallest_valid_set {
                len_of_smallest_valid_set = subset.len();
                smallest_valid_quantum_entaglement =
                    smallest_valid_quantum_entaglement.min(subset.iter().copied().product());
            }
        }
    }

    smallest_valid_quantum_entaglement
}

#[test]
fn both_parts() {
    let sample_packages = parse_packages(SAMPLE);
    assert_eq!(99, find_lowest_config(&sample_packages, 3));
    assert_eq!(44, find_lowest_config(&sample_packages, 4));

    let input_packages = parse_packages(INPUT);
    assert_eq!(10439961859, find_lowest_config(&input_packages, 3));
    assert_eq!(72050269, find_lowest_config(&input_packages, 4));
}
