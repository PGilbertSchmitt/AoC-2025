use std::collections::VecDeque;

use fxhash::FxHashMap;

use crate::util::{generate_hash, generate_padded_hash};

type HashArr = [u8; 32];

fn hash_form(hash: [u8; 16]) -> String {
    let mut output = String::new();
    for byte in hash {
        output.push_str(&format!("{:0>2x}", byte));
    }
    output
}

fn generate_expensive_hash(base: &[u8], padding: usize) -> [u8; 16] {
    let mut hash: [u8; 16] = generate_padded_hash(base, padding);
    for _ in 0..2016 {
        hash = generate_hash(&hash_form(hash).as_bytes());
    }
    hash
}

fn easier_hash(hash: [u8; 16]) -> HashArr {
    let mut output = [0; 32];

    for (i, byte) in hash.into_iter().enumerate() {
        let first = byte >> 4;
        let second = byte & 0x0F;
        output[i * 2] = first;
        output[i * 2 + 1] = second;
    }

    output
}

fn find_relevant_details(hash: &HashArr) -> (Option<u8>, Vec<u8>) {
    let mut count = 1;
    let mut cur_match: u8 = hash[0];
    let mut triplet: Option<u8> = None;
    let mut quints = Vec::new();

    for half_byte in &hash[1..] {
        if *half_byte == cur_match {
            count += 1;

            if count == 3 && triplet.is_none() {
                triplet = Some(*half_byte);
            }

            if count == 5 {
                quints.push(*half_byte);
            }
        } else {
            cur_match = *half_byte;
            count = 1;
        }
    }

    (triplet, quints)
}

struct FutureSearchState {
    index: usize,
    quints: FxHashMap<usize, Vec<u8>>,
}

struct TripletKey {
    index: usize,
    character: u8,
}

fn confirm_key(
    input: &[u8],
    current_key: &TripletKey,
    future_search_state: &mut FutureSearchState,
    key_queue: &mut VecDeque<TripletKey>,
    expensive: bool,
) -> bool {
    // Search existing keys first
    for index in (current_key.index + 1)..future_search_state.index {
        if let Some(quints) = future_search_state.quints.get(&index) {
            if quints.contains(&current_key.character) {
                return true;
            }
        }
    }

    // None found, generate new keys up to the limit
    let max_search_index = current_key.index + 1001; // One more than a thousand since Rust ranges are [x..y)

    for index in future_search_state.index..max_search_index {
        future_search_state.index += 1;
        let hash = if expensive {
            generate_expensive_hash(input, index)
        } else {
            generate_padded_hash(input, index)
        };
        let hash = easier_hash(hash);
        let (triplet_hex, quints) = find_relevant_details(&hash);

        triplet_hex.map(|hex| {
            key_queue.push_back(TripletKey {
                index,
                character: hex,
            });
        });

        if !quints.is_empty() {
            future_search_state.quints.insert(index, quints.clone());
            if quints.contains(&current_key.character) {
                return true;
            }
        }
    }

    false
}

// Not even sure if this is necessary
fn find_next_potential_key(input: &[u8], index: &mut usize, expensive: bool) -> TripletKey {
    loop {
        let hash = if expensive {
            generate_expensive_hash(input, *index)
        } else {
            generate_padded_hash(input, *index)
        };
        let hash = easier_hash(hash);
        // Any quints found here are not needed since this function is only called
        // when the triplet is needed, and thus a quint wouldn't confirm anything
        let (triplet_hex, _) = find_relevant_details(&hash);

        *index += 1;
        if let Some(hex) = triplet_hex {
            return TripletKey {
                index: *index,
                character: hex,
            };
        }
    }
}

fn find_keys(input: &[u8], expensive: bool) -> usize {
    let mut key_queue: VecDeque<TripletKey> = VecDeque::new();

    // Can't start until we have the first potential key
    let mut index = 0;
    key_queue.push_back(find_next_potential_key(input, &mut index, expensive));

    let mut found_keys: Vec<usize> = Vec::new();
    let mut future_keys = FutureSearchState {
        index: index + 1,
        quints: FxHashMap::default(),
    };

    while found_keys.len() < 64 {
        let current_key = match key_queue.pop_front() {
            Some(key) => {
                index = key.index + 1;
                key
            }
            None => find_next_potential_key(input, &mut index, expensive),
        };
        if confirm_key(
            input,
            &current_key,
            &mut future_keys,
            &mut key_queue,
            expensive,
        ) {
            found_keys.push(current_key.index);
        }
    }

    found_keys.pop().unwrap()
}

#[test]
fn part1() {
    assert_eq!(22728, find_keys(b"abc", false));
    assert_eq!(15035, find_keys(b"ihaygndm", false));
}

#[test]
fn part2() {
    // Takes over a minute to evalute in release mode, and over 3 minutes in debug mode
    // assert_eq!(22551, find_keys(b"abc", true));
    // assert_eq!(19968, find_keys(b"ihaygndm", true));
}
