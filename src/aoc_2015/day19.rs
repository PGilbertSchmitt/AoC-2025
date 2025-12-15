use itertools::Itertools;
use logos::Logos;
use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    usize,
};

const SAMPLE: &str = "
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

const INPUT: &str = include_str!("./inputs/day19.txt");

type TransformMap = HashMap<String, Vec<String>>;

#[derive(Logos)]
#[logos(skip "[ =>\n]")]
enum Token {
    #[regex("[A-Za-z]+", |lex| lex.slice().to_owned())]
    Word(String),
}

impl Token {
    fn take(self) -> String {
        let Self::Word(word) = self;
        word
    }
}

fn build_transform_map(input: &str) -> (String, TransformMap) {
    let mut h: TransformMap = HashMap::new();

    let mut parts = input.trim().split("\n\n");
    let mappings = parts.next().unwrap();
    let init_sequence = parts.next().unwrap();

    let mut lex = Token::lexer(mappings);
    while let Some(tok) = lex.next() {
        let key = tok.unwrap().take();
        let val = lex.next().unwrap().unwrap().take();
        match h.entry(key) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(val);
            }
            Entry::Vacant(e) => {
                e.insert(vec![val]);
            }
        }
    }

    (init_sequence.to_owned(), h)
}

fn transformations(sequence: String, map: &TransformMap) -> HashSet<String> {
    let mut s = HashSet::<String>::new();

    for idx in 0..sequence.len() {
        let portion_1 = &sequence[idx..idx + 1];

        if let Some(inserts) = map.get(portion_1) {
            let start = &sequence[..idx];
            let end = &sequence[idx + 1..];
            for insert in inserts {
                s.insert(format!("{start}{insert}{end}"));
            }
        }

        if idx + 1 < sequence.len() {
            let portion_2 = &sequence[idx..idx + 2];
            if let Some(inserts) = map.get(portion_2) {
                let start = &sequence[..idx];
                let end = &sequence[idx + 2..];
                for insert in inserts {
                    s.insert(format!("{start}{insert}{end}"));
                }
            }
        }
    }

    s
}

fn calibrate(input: &str) -> usize {
    let (sequence, map) = build_transform_map(input);
    let results = transformations(sequence, &map);
    results.len()
}

fn fabricate(input: &str) -> usize {
    let (sequence, map) = build_transform_map(input);
    let mut inverse_map = HashMap::<String, String>::new();
    for (reduced_molecule, original_molecules) in map.into_iter() {
        for original_molecule in original_molecules {
            inverse_map.insert(original_molecule, reduced_molecule.clone());
        }
    }

    let (steps, molecule) = reduce_compound(&sequence, &inverse_map);
    let (last_steps, ..) = reduce_molecule(&molecule, &inverse_map, |m| m == "e");

    steps + last_steps
}

fn reduce_compound(substr: &str, lookup: &HashMap<String, String>) -> (usize, String) {
    let mut steps = 0;

    let mut compound = substr.to_owned();

    while compound.contains("Ar") {
        // Must also contain "Rn"
        let argon_idx = compound.find("Ar").unwrap();
        let radon_idx = compound[0..argon_idx].rfind("Rn").unwrap();
        let prev_radon_idx_opt = compound[0..radon_idx - 1].rfind("Rn");
        let molecule = if let Some(prev_radon_idx) = prev_radon_idx_opt {
            &compound[prev_radon_idx + 2..argon_idx + 2]
        } else {
            &compound[0..argon_idx + 2]
        };
        let (sub_steps, reduction) = reduce_molecule(molecule, lookup, |m| !m.contains("Ar"));
        steps += sub_steps;
        compound = if let Some(prev_radon_idx) = prev_radon_idx_opt {
            format!(
                "{}{}{}",
                &compound[0..prev_radon_idx + 2],
                reduction,
                &compound[argon_idx + 2..]
            )
        } else {
            format!("{}{}", reduction, &compound[argon_idx + 2..])
        };
    }

    (steps, compound)
}

fn reduce_molecule<F>(molecule: &str, lookup: &HashMap<String, String>, until: F) -> (usize, String)
where
    F: Fn(&str) -> bool,
{
    let mut map = HashMap::<String, usize>::new();
    map.insert(molecule.to_string(), 0);

    loop {
        let item = map
            .keys()
            .sorted_unstable_by(|a, b| a.len().cmp(&b.len()))
            .next()
            .unwrap()
            .clone();
        let iterations = *map.get(&item).unwrap();
        map.remove(&item);

        for (original_material, reduced_material) in lookup.iter() {
            if let Some(found_idx) = item.find(original_material) {
                let new_molecule = format!(
                    "{}{}{}",
                    &item[0..found_idx],
                    reduced_material,
                    &item[found_idx + original_material.len()..]
                );
                if until(&new_molecule) {
                    return (iterations + 1, new_molecule);
                }
                map.insert(new_molecule, iterations + 1);
            }
        }
    }
}

#[test]
fn part_1() {
    assert_eq!(7, calibrate(SAMPLE));
    assert_eq!(518, calibrate(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(6, fabricate(SAMPLE));
    assert_eq!(200, fabricate(INPUT));
}
