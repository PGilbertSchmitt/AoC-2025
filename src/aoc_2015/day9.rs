use std::{collections::HashMap, usize};

use itertools::Itertools;
use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[ \n=]", skip "to")]
enum Token {
    #[token("Faerun")]
    Faerun,

    #[token("Tristram")]
    Tristram,

    #[token("Tambi")]
    Tambi,

    #[token("Norrath")]
    Norrath,

    #[token("Snowdin")]
    Snowdin,

    #[token("Straylight")]
    Straylight,

    #[token("AlphaCentauri")]
    AlphaCentauri,

    #[token("Arbre")]
    Arbre,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<usize>().unwrap())]
    Distance(usize),
}

impl Token {
    fn take_city(self) -> City {
        match self {
            Self::Faerun => City::Faerun,
            Self::Tristram => City::Tristram,
            Self::Tambi => City::Tambi,
            Self::Norrath => City::Norrath,
            Self::Snowdin => City::Snowdin,
            Self::Straylight => City::Straylight,
            Self::AlphaCentauri => City::AlphaCentauri,
            Self::Arbre => City::Arbre,
            Self::Distance(_) => panic!("Expected city, found distance"),
        }
    }

    fn take_dist(self) -> usize {
        match self {
            Self::Distance(dist) => dist,
            _ => panic!("Expected distance, found city"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum City {
    Faerun,
    Tristram,
    Tambi,
    Norrath,
    Snowdin,
    Straylight,
    AlphaCentauri,
    Arbre,
}

impl City {
    fn all() -> Vec<City> {
        vec![
            City::Faerun,
            City::Tristram,
            City::Tambi,
            City::Norrath,
            City::Snowdin,
            City::Straylight,
            City::AlphaCentauri,
            City::Arbre,
        ]
    }
}

fn build_dist_map(input: &str) -> HashMap<(City, City), usize> {
    let mut h = HashMap::new();

    let mut lex = Token::lexer(input);
    while let Some(city_1) = lex.next() {
        let city_1 = city_1.unwrap().take_city();
        let city_2 = lex.next().unwrap().unwrap().take_city();
        let dist = lex.next().unwrap().unwrap().take_dist();

        h.insert((city_1, city_2), dist);
        h.insert((city_2, city_1), dist);
    }

    h
}

fn route_cost(route: Vec<City>, lookup: &HashMap<(City, City), usize>) -> usize {
    route.windows(2).fold(0, |sum, win| {
        sum + *lookup.get(&(win[0].clone(), win[1].clone())).unwrap()
    })
}

fn find_cheapest_route_cost(lookup: &HashMap<(City, City), usize>) -> usize {
    let mut min_cost = usize::MAX;
    for perm in City::all().into_iter().permutations(8) {
        let cost = route_cost(perm, lookup);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn find_highest_route_cost(lookup: &HashMap<(City, City), usize>) -> usize {
    let mut max_cost = 0;
    for perm in City::all().into_iter().permutations(8) {
        let cost = route_cost(perm, lookup);
        if cost > max_cost {
            max_cost = cost;
        }
    }
    max_cost
}

const INPUT: &'static str = include_str!("./inputs/day9.txt");

#[test]
fn both_parts() {
    let lookup = build_dist_map(INPUT);
    assert_eq!(117, find_cheapest_route_cost(&lookup));
    assert_eq!(909, find_highest_route_cost(&lookup));
}
