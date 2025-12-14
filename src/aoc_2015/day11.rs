enum Inc {
    Normal(u8),
    Carry(u8),
}

fn increment_single_value(value: u8) -> Inc {
    // i, l, o
    if value == 9 || value == 12 || value == 15 {
        return Inc::Normal(value + 2);
    }

    // z
    if value == 26 {
        return Inc::Carry(1);
    }

    Inc::Normal(value + 1)
}

fn increment(password: &mut Vec<u8>) {
    for letter in password.iter_mut().rev() {
        match increment_single_value(*letter) {
            Inc::Normal(value) => {
                *letter = value;
                break;
            }
            Inc::Carry(value) => {
                *letter = value;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<u8> {
    input.chars().map(|ch| (ch as u8) - 96).collect()
}

fn pass_string(vec: &Vec<u8>) -> String {
    vec.into_iter().map(|ch| (ch + 96) as char).collect()
}

fn valid_password(password: &Vec<u8>) -> bool {
    let mut last: Option<u8> = None;

    let mut increment_chain = 0;
    let mut increment_reached = false;

    let mut pair_idxs = Vec::new();

    for (idx, ch) in password.iter().enumerate() {
        // Chain
        if last == Some(*ch - 1) {
            increment_chain += 1;
            if increment_chain >= 2 {
                increment_reached = true;
            }
        } else {
            increment_chain = 0;
        }

        // Doubles
        if last == Some(*ch) {
            pair_idxs.push(idx);
        }

        last = Some(*ch)
    }

    if pair_idxs.len() >= 2 {
        let first_pair_idx = *pair_idxs.first().unwrap();
        let last_pair_idx = *pair_idxs.last().unwrap();
        let non_overlapping_pairs = (last_pair_idx - first_pair_idx) >= 2;
        increment_reached && non_overlapping_pairs
    } else {
        false
    }
}

#[test]
fn solutions() {
    let mut pass = parse_input("vzbxkghb");
    while !valid_password(&pass) {
        increment(&mut pass);
    }
    assert_eq!("vzbxxyzz", pass_string(&pass), "part 1");
    increment(&mut pass);
    while !valid_password(&pass) {
        increment(&mut pass);
    }
    assert_eq!("vzcaabcc", pass_string(&pass), "part 2");
}
