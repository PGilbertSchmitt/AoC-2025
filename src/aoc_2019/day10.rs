use std::f64::consts::TAU;

use fxhash::{FxHashMap, FxHashSet};

use crate::util::{Pair, lowest_common_factor};

const SAMPLE_1: &str = "
.#..#
.....
#####
....#
...##";

const SAMPLE_2: &str = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

const SAMPLE_3: &str = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

const INPUT: &str = include_str!("./inputs/day10.txt");

fn parse_asteroids(input: &str) -> Vec<Pair> {
    let mut asteroids = Vec::new();
    for (y, line) in input.trim().split('\n').enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.push(Pair(x as i64, y as i64));
            }
        }
    }
    asteroids
}

fn check_first_sightlines(
    factor_cache: &FactorCache,
    asteroids: &Vec<Pair>,
) -> FxHashSet<(Pair, Pair)> {
    let asteroid_cache = asteroids.iter().copied().collect::<FxHashSet<Pair>>();
    let mut sightlines = FxHashSet::default();

    for (i, point) in asteroids.iter().enumerate() {
        for other in &asteroids[i + 1..] {
            let seen = sightline_first(&factor_cache, &asteroid_cache, point, other);

            sightlines.insert((seen, *point));
            sightlines.insert((*point, seen));
        }
    }

    sightlines
}

// I feel like there's a better way to sort coordinates clockwise from noon by not using floats
fn clockwise_rads(x: i64, y: i64) -> f64 {
    let rads = (x as f64).atan2(-y as f64);
    if rads >= 0.0 { rads } else { rads + TAU }
}

fn nth_rotational_sightline(
    from: &Pair,
    factor_cache: &FactorCache,
    asteroids: &Vec<Pair>,
    n: usize,
) -> Option<Pair> {
    let asteroid_cache = asteroids.iter().copied().collect::<FxHashSet<Pair>>();
    let mut visited = FxHashSet::<Pair>::default();
    let mut sorted_sightlines = Vec::<(f64, Pair)>::new();

    for other in asteroids.iter() {
        if from != other && !visited.contains(&other) {
            let seen_list = sightline_vec(&factor_cache, &asteroid_cache, from, other);
            let diff_x = other.0 - from.0;
            let diff_y = other.1 - from.1;
            // All items in the seen list would have the exact same angle, so we only need to calculate it once
            let angle = clockwise_rads(diff_x, diff_y);
            for (i, asteroid) in seen_list.iter().enumerate() {
                // Since all asteroids in the same vector (in both senses of the word) will have the same angle,
                // we need to differentiate them by their order. An asteroid can't be reached until all asteroids
                // before it have been removed, which means an additional rotation per prior asteroid. By adding
                // 2*Tau for each asteroid, we push the asteroid down the queue.
                let sort_order = angle + (TAU * (i as f64));
                visited.insert(*asteroid);
                sorted_sightlines.push((sort_order, *asteroid));
            }
        }
    }

    sorted_sightlines.sort_by(|this, other| this.0.total_cmp(&other.0));
    sorted_sightlines.iter().nth(n).map(|(_, pair)| *pair)
}

fn sightline_first(
    factor_cache: &FactorCache,
    asteroids: &FxHashSet<Pair>,
    &Pair(ax, ay): &Pair,
    &Pair(bx, by): &Pair,
) -> Pair {
    let x_diff = bx - ax;
    let y_diff = by - ay;
    if let Some(factor) = factor_cache.get(&(x_diff.abs(), y_diff.abs())) {
        let mut cur_coord = Pair(ax, ay);
        let step = Pair(x_diff / factor, y_diff / factor);
        for _ in 0..*factor - 1 {
            cur_coord = cur_coord + step;
            if asteroids.contains(&cur_coord) {
                return cur_coord;
            }
        }
    }

    Pair(bx, by)
}

fn sightline_vec(
    factor_cache: &FactorCache,
    asteroids: &FxHashSet<Pair>,
    &Pair(ax, ay): &Pair,
    &Pair(bx, by): &Pair,
) -> Vec<Pair> {
    let x_diff = bx - ax;
    let y_diff = by - ay;
    let mut sightline_list = Vec::new();
    let factor = factor_cache
        .get(&(x_diff.abs(), y_diff.abs()))
        .unwrap_or(&1);

    let mut cur_coord = Pair(ax, ay);
    let step = Pair(x_diff / factor, y_diff / factor);
    loop {
        cur_coord = cur_coord + step;
        if cur_coord.0 < 0 || cur_coord.0 > 36 || cur_coord.1 < 0 || cur_coord.1 > 36 {
            break;
        }
        if asteroids.contains(&cur_coord) {
            sightline_list.push(cur_coord);
        }
    }
    sightline_list
}

fn count_sightlines(pairs: &FxHashSet<(Pair, Pair)>) -> (u64, Pair) {
    let mut tally = FxHashMap::<&Pair, u64>::default();
    for (a, _b) in pairs {
        tally.entry(a).and_modify(|v| *v += 1).or_insert(1);
    }

    tally
        .into_iter()
        .fold((0, Pair(-1, -1)), |acc, (&next_pair, count)| {
            if count > acc.0 {
                (count, next_pair)
            } else {
                acc
            }
        })
}

type FactorCache = FxHashMap<(i64, i64), i64>;

fn pregen_lowest_common_factors() -> FactorCache {
    let mut map = FxHashMap::default();
    for x in 0..40 {
        for y in x..40 {
            if let Some(factor) = lowest_common_factor(x, y) {
                map.insert((x, y), factor);
                map.insert((y, x), factor);
            }
        }
    }
    map
}

#[test]
fn samples() {
    let factor_cache = pregen_lowest_common_factors();

    assert_eq!(
        8,
        count_sightlines(&check_first_sightlines(
            &factor_cache,
            &parse_asteroids(SAMPLE_1)
        ))
        .0
    );
    assert_eq!(
        33,
        count_sightlines(&check_first_sightlines(
            &factor_cache,
            &parse_asteroids(SAMPLE_2)
        ))
        .0
    );

    let asteroids_3 = parse_asteroids(SAMPLE_3);
    let (sightlines, station_coordinates) =
        count_sightlines(&check_first_sightlines(&factor_cache, &asteroids_3));
    assert_eq!(210, sightlines);
    let two_hundreth =
        nth_rotational_sightline(&station_coordinates, &factor_cache, &asteroids_3, 199);
    assert_eq!(Some(802), two_hundreth.map(|Pair(x, y)| x * 100 + y));
}

#[test]
fn solutions() {
    let factor_cache = pregen_lowest_common_factors();

    let asteroids = parse_asteroids(INPUT);
    let (max_sightlines, station_coordinates) =
        count_sightlines(&check_first_sightlines(&factor_cache, &asteroids));
    assert_eq!(276, max_sightlines);
    let two_hundreth =
        nth_rotational_sightline(&station_coordinates, &factor_cache, &asteroids, 199);
    assert_eq!(Some(1321), two_hundreth.map(|Pair(x, y)| x * 100 + y));
}
