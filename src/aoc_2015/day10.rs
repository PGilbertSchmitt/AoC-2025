fn expand(sequence: Vec<u8>) -> Vec<u8> {
    let mut last: Option<u8> = None;
    let mut count = 1;
    let mut output: Vec<u8> = Vec::new();
    for num in sequence.into_iter() {
        let opt = Some(num);

        if opt == last {
            count += 1
        } else if let Some(last) = last {
            output.push(count);
            output.push(last);
            count = 1;
        }

        last = opt;
    }

    output.push(count);
    output.push(last.unwrap());

    output
}

fn expand_n_times(n: u8, mut seq: Vec<u8>) -> usize {
    for _ in 0..n {
        seq = expand(seq);
    }
    seq.len()
}

#[test]
fn both_parts() {
    let input: Vec<u8> = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
    assert_eq!(492982, expand_n_times(40, input.clone()));
    assert_eq!(6989950, expand_n_times(50, input));
}
