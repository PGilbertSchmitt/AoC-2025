fn parse_containers(input: &str) -> Vec<usize> {
    let mut v: Vec<usize> = input
        .trim()
        .split('\n')
        .map(|seg| seg.parse::<usize>().unwrap())
        .collect();
    v.sort();
    v.reverse();
    v
}

fn count_containers(
    space_remaining: usize,
    depth: usize,
    containers: &[usize],
    tracker: &mut (usize, usize),
) -> usize {
    let mut total = 0;
    let new_depth = depth + 1;

    for idx in 0..containers.len() {
        let cur_container = containers[idx];

        if cur_container > space_remaining {
            continue;
        }

        if cur_container == space_remaining {
            total += 1;
            if new_depth < tracker.0 {
                tracker.0 = new_depth;
                tracker.1 = 1;
            } else if new_depth == tracker.0 {
                tracker.1 += 1;
            }
            continue;
        }

        let cur_remaining = space_remaining - cur_container;
        let rest = &containers[idx + 1..];

        let inner_total = count_containers(cur_remaining, new_depth, rest, tracker);
        total += inner_total;
    }

    total
}

const INPUT: &str = include_str!("./inputs/day17.txt");

#[test]
fn solutions() {
    let containers = parse_containers(INPUT);
    let mut tracker = (usize::MAX, 0);
    assert_eq!(654, count_containers(150, 0, &containers, &mut tracker));
    assert_eq!(57, tracker.1);
}
