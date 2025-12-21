use fxhash::FxHashMap;

const SAMPLE_1: &str = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

const SAMPLE_2: &str = "
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

const INPUT: &str = include_str!("./inputs/day6.txt");

type OrbitMap = FxHashMap<String, String>;
type TransferCache = FxHashMap<String, u64>;

fn parse_orbits(input: &str) -> OrbitMap {
    input
        .trim()
        .split('\n')
        .fold(FxHashMap::default(), |mut acc, line| {
            let mut parts = line.split(')');
            let parent = parts.next().unwrap().to_string();
            let child = parts.next().unwrap().to_string();
            acc.insert(child, parent);
            acc
        })
}

fn calculate_com_orbit(body: &str, orbits: &OrbitMap, cache: &mut TransferCache) -> u64 {
    if body == "COM" {
        0
    } else {
        let next_body = orbits.get(body).unwrap();
        let cur_orbit = if let Some(cached_orbits) = cache.get(next_body) {
            cached_orbits + 1
        } else {
            let to_next = calculate_com_orbit(next_body, orbits, cache);
            // println!("Cache {body} with value {cur_len}");
            to_next + 1
        };
        cache.insert(body.to_string(), cur_orbit);
        cur_orbit
    }
}

fn tally_orbits(orbits: &OrbitMap) -> (u64, TransferCache) {
    let mut cache = TransferCache::default();
    let tally = orbits
        .keys()
        .map(|body| calculate_com_orbit(body, orbits, &mut cache))
        .sum();
    (tally, cache)
}

fn shortest_transfer(orbits: &OrbitMap, cache: &TransferCache) -> u64 {
    let start = "YOU";
    let end = "SAN";
    let start_total = cache.get(start).unwrap();
    let end_total = cache.get(end).unwrap();
    let (mut shorter_orbit, longer_orbit, diff) = if start_total < end_total {
        (start, end, end_total - start_total)
    } else {
        (end, start, start_total - end_total)
    };
    let mut equal_orbit = longer_orbit;
    for _ in 0..diff {
        equal_orbit = &orbits.get(equal_orbit).unwrap();
    }
    let mut counter = diff;
    while shorter_orbit != equal_orbit {
        counter += 2;
        shorter_orbit = orbits.get(shorter_orbit).unwrap();
        equal_orbit = orbits.get(equal_orbit).unwrap();
    }

    counter - 2
}

#[test]
fn samples() {
    let orbits = parse_orbits(SAMPLE_1);
    let (count, _) = tally_orbits(&orbits);
    assert_eq!(42, count);

    let orbits = parse_orbits(SAMPLE_2);
    let (_, cache) = tally_orbits(&orbits);
    assert_eq!(4, shortest_transfer(&orbits, &cache));
}

#[test]
fn solutions() {
    let orbits = parse_orbits(INPUT);
    let (count, cache) = tally_orbits(&orbits);
    assert_eq!(162439, count);
    assert_eq!(367, shortest_transfer(&orbits, &cache));
}
