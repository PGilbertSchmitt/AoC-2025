type Data = Vec<bool>;

fn data_str(data: &Data) -> String {
    let mut s = String::new();
    for d in data {
        s.push(if *d { '1' } else { '0' });
    }
    s
}

fn parse_data(input: &str) -> Data {
    input.chars().map(|ch| ch == '1').collect()
}

fn dragon_curve_step(data: &mut Data) {
    let mut second_section: Data = data.iter().map(|bit| !bit).rev().collect();
    data.push(false);
    data.append(&mut second_section);
}

fn checksum(data: Data) -> Data {
    if data.len() % 2 != 0 {
        panic!("Can't create checksum for data of len {}", data.len());
    }

    let mut next_data: Data = Vec::new();
    let mut bits = data.iter();
    while let Some(bit_1) = bits.next() {
        let bit_2 = bits.next().unwrap();
        next_data.push(*bit_1 == *bit_2);
    }

    if next_data.len() % 2 == 0 {
        return checksum(next_data);
    }
    next_data
}

fn generate_dummy_checksum(input: &str, len: usize) -> String {
    let mut data = parse_data(input);
    while data.len() < len {
        dragon_curve_step(&mut data);
    }
    data.truncate(len);
    data_str(&checksum(data))
}

const INPUT: &str = "01110110101001000";

#[test]
fn both_parts() {
    assert_eq!("11100111011101111", generate_dummy_checksum(INPUT, 272));
    // This could maybe have been done faster, but it was not immediately clear to me how,
    // and 1 second of runtime in debug mode is pretty nice (0.07s in release mode).
    assert_eq!(
        "10001110010000110",
        generate_dummy_checksum(INPUT, 35651584)
    );
}
