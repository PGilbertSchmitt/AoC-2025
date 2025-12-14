use std::collections::HashMap;

use itertools::Itertools;
use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[\n. ]", skip r"happiness units by sitting next to")]
enum Token {
    #[regex("[A-Z][a-z]+", |lex| lex.slice().to_string())]
    Name(String),

    #[token("would gain")]
    Gain,

    #[token("would lose")]
    Lose,

    #[regex("[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    Value(i32),
}

impl Token {
    fn take_name(self) -> Name {
        match self {
            Self::Name(name) => name.into(),
            _ => panic!("Not a name"),
        }
    }

    fn take_value(self) -> i32 {
        match self {
            Self::Value(val) => val,
            _ => panic!("Not a value"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Name {
    Alice,
    Bob,
    Carol,
    David,
    Eric,
    Frank,
    George,
    Mallory,

    // It's me, I'm the dud
    Dud,
}

impl Name {
    // Unlike Day 9, the permutations also require us to start and end on the same name,
    // therefore it forms a perfect loop. That means we can reduce the total number of
    // permutations by 8 (the count of elements) by keeping one name positioned statically
    // (Alice) and only caring about the relative permutations of the remaining 7 names.
    fn permutations() -> Vec<Vec<Name>> {
        let base = vec![
            Self::Bob,
            Self::Carol,
            Self::David,
            Self::Eric,
            Self::Frank,
            Self::George,
            Self::Mallory,
        ];

        base.into_iter()
            .permutations(7)
            .map(|mut v| {
                v.push(Self::Alice);
                v
            })
            .collect()
    }

    fn permutations_with_dud() -> Vec<Vec<Name>> {
        let base = vec![
            Self::Alice,
            Self::Bob,
            Self::Carol,
            Self::David,
            Self::Eric,
            Self::Frank,
            Self::George,
            Self::Mallory,
        ];

        base.into_iter()
            .permutations(8)
            .map(|mut v| {
                v.push(Self::Dud);
                v
            })
            .collect()
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "Alice" => Self::Alice,
            "Bob" => Self::Bob,
            "Carol" => Self::Carol,
            "David" => Self::David,
            "Eric" => Self::Eric,
            "Frank" => Self::Frank,
            "George" => Self::George,
            "Mallory" => Self::Mallory,
            _ => panic!("'{value}' is not a registered name"),
        }
    }
}

type NameMap = HashMap<(Name, Name), i32>;

fn parse_happiness_map(input: &str) -> NameMap {
    let mut map: NameMap = HashMap::new();

    let mut lex = Token::lexer(input);
    while let Some(token) = lex.next() {
        let first = token.unwrap().take_name();

        let mult = match lex.next().unwrap().unwrap() {
            Token::Gain => 1,
            Token::Lose => -1,
            _ => panic!("Expected 'gain' or 'lose' section"),
        };

        let value = mult * lex.next().unwrap().unwrap().take_value();
        let second = lex.next().unwrap().unwrap().take_name();

        map.insert((first, second), value);
    }

    map
}

// Just don't use this with the dud and you're gucci
fn get_happiness(name_1: &Name, name_2: &Name, map: &NameMap) -> i32 {
    let x = *map.get(&(name_1.clone(), name_2.clone())).unwrap();
    let y = *map.get(&(name_2.clone(), name_1.clone())).unwrap();
    x + y
}

fn total_happiness(mut table: Vec<Name>, map: &NameMap, with_dud: bool) -> i32 {
    let init = if with_dud {
        0
    } else {
        get_happiness(table.first().unwrap(), table.last().unwrap(), map)
    };

    if with_dud {
        table.pop();
    }

    let total = table
        .windows(2)
        .fold(init, |sum, win| sum + get_happiness(&win[0], &win[1], map));

    total
}

const INPUT: &str = include_str!("./inputs/day13.txt");

#[test]
fn part_1() {
    let map = parse_happiness_map(INPUT);
    let mut optimal_happiness = 0;
    for perm in Name::permutations() {
        optimal_happiness = total_happiness(perm, &map, false).max(optimal_happiness);
    }
    assert_eq!(709, optimal_happiness);
}

#[test]
fn part_2() {
    let map = parse_happiness_map(INPUT);
    let mut optimal_happiness = 0;
    for perm in Name::permutations_with_dud() {
        optimal_happiness = total_happiness(perm, &map, true).max(optimal_happiness);
    }
    assert_eq!(668, optimal_happiness);
}
