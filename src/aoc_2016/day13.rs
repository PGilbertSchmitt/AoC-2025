use std::collections::{VecDeque, binary_heap::BinaryHeap};

use fxhash::FxHashSet;

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn is_space(&self, input: usize) -> bool {
        let x = self.0;
        let y = self.1;
        let total = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + input;
        total.count_ones() % 2 == 0
    }

    fn man_dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(PartialEq, Eq)]
struct Space {
    cost: usize,
    heuristic: usize,
    position: Point,
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // f(n) = g(n) + h(n), so compare cost plus heuristic against other points
        // Reversed so that the BinaryHeap becomes a min-heap instead of a max-heap
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(point: &Point) -> Vec<Point> {
    let x = point.0;
    let y = point.1;
    let mut ns = vec![Point(x + 1, y), Point(x, y + 1)];
    // ,
    // Point (x, y-1),
    if x > 0 {
        ns.push(Point(x - 1, y));
    }
    if y > 0 {
        ns.push(Point(x, y - 1));
    }

    ns
}

const START: Point = Point(1, 1);
// const END: Point = Point (31, 39);

fn find_shortest_path(input: usize, end: Point) -> usize {
    let mut visited: FxHashSet<Point> = FxHashSet::default();
    let mut queue: BinaryHeap<Space> = BinaryHeap::new();
    queue.push(Space {
        cost: 0,
        position: START,
        heuristic: START.man_dist(&end),
    });

    // The map can go on forever, so queue will never be empty. This should at least
    // keep us from hitting any infinite loops
    while visited.len() < 1_000_000 || queue.is_empty() {
        let next = queue.pop().unwrap();
        if next.position == end {
            return next.cost;
        }

        let cost = next.cost + 1;
        for neighbor in get_neighbors(&next.position) {
            if neighbor.is_space(input) && !visited.contains(&neighbor) {
                queue.push(Space {
                    cost,
                    heuristic: neighbor.man_dist(&end),
                    position: neighbor,
                });
            }
        }

        visited.insert(next.position);
    }

    panic!(
        "Could not find path to END, visited {} spaces",
        visited.len()
    );
}

struct FillSpace {
    cost: usize,
    position: Point,
}

fn count_all_spaces_within(input: usize, max_cost: usize) -> usize {
    let mut visited: FxHashSet<Point> = FxHashSet::default();
    let mut stack: VecDeque<FillSpace> = VecDeque::new();

    stack.push_back(FillSpace {
        cost: 0,
        position: START,
    });
    while !stack.is_empty() {
        let next = stack.pop_front().unwrap();

        if next.cost < max_cost {
            let new_cost = next.cost + 1;
            for neighbor in get_neighbors(&next.position) {
                if neighbor.is_space(input) && !visited.contains(&neighbor) {
                    stack.push_back(FillSpace {
                        cost: new_cost,
                        position: neighbor,
                    });
                }
            }
        }

        visited.insert(next.position);
    }

    visited.len()
}

#[test]
fn part1() {
    // Sample
    assert_eq!(11, find_shortest_path(10, Point(7, 4)));

    // Actual input
    assert_eq!(90, find_shortest_path(1352, Point(31, 39)));
}

#[test]
fn part2() {
    assert_eq!(135, count_all_spaces_within(1352, 50));
}
