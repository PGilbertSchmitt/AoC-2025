use std::collections::HashMap;

fn is_nice(word: &[u8]) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut last_ch: Option<char> = None;

    for ch in word.iter() {
        let ch = (*ch) as char;

        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            vowel_count += 1;
        }

        if Some(ch) == last_ch {
            has_double = true;
        }

        if let Some(last_ch) = last_ch {
            let pair = (last_ch, ch);
            if pair == ('a', 'b') || pair == ('c', 'd') || pair == ('p', 'q') || pair == ('x', 'y')
            {
                return false;
            }
        }

        last_ch = Some(ch);
    }

    has_double && vowel_count >= 3
}

fn count_nice(input: &[u8]) -> usize {
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .filter(|word| is_nice(*word))
        .count()
}

type Pair = (u8, u8);

fn is_really_nice(word: &[u8]) -> bool {
    let mut letter_pairs: HashMap<Pair, Vec<usize>> = HashMap::new();

    letter_pairs.insert((word[0], word[1]), vec![0]);

    let mut found_disjoint_pair = false;
    let mut found_sandwich = false;

    for (idx, window) in word.windows(3).enumerate() {
        let a = window[0];
        let b = window[1];
        let c = window[2];

        let new_pair = (b, c);
        let new_pair_idx = idx + 1;
        letter_pairs
            .entry(new_pair)
            .and_modify(|v| {
                if v.iter().any(|prev| new_pair_idx - *prev > 1) {
                    found_disjoint_pair = true;
                }
                v.push(new_pair_idx);
            })
            .or_insert(vec![new_pair_idx]);

        if a == c {
            found_sandwich = true;
        }

        if found_disjoint_pair && found_sandwich {
            return true;
        }
    }

    false
}

fn count_really_nice(input: &[u8]) -> usize {
    input
        .trim_ascii()
        .split(|c| *c == b'\n')
        .filter(|word| is_really_nice(*word))
        .count()
}

const SAMPLE_1: &[u8] = b"ugknbfddgicrmopn\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb\n";
const SAMPLE_2: &[u8] = b"qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy\n";
const INPUT: &[u8] = include_bytes!("./inputs/day5.txt");

#[test]
fn part_1() {
    assert_eq!(1, count_nice(SAMPLE_1));
    assert_eq!(236, count_nice(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(2, count_really_nice(SAMPLE_2));
    assert_eq!(51, count_really_nice(INPUT));
}
