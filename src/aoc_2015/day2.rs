use std::str;

fn dimension_to_int(part: Option<&[u8]>) -> usize {
    str::parse(str::from_utf8(part.unwrap()).unwrap()).unwrap_or(0)
}

fn total_materials(input: &[u8]) -> (usize, usize) {
    input
        .trim_ascii()
        .split(|b| *b == b'\n')
        .fold((0, 0), |(paper, ribbon), line| {
            let mut dimension_iter = line.trim_ascii().split(|b| *b == b'x');
            let l = dimension_to_int(dimension_iter.next());
            let w = dimension_to_int(dimension_iter.next());
            let h = dimension_to_int(dimension_iter.next());

            let mut dimensions = vec![l, w, h];
            dimensions.sort();
            let first_smallest = dimensions.get(0).unwrap();
            let second_smallest = dimensions.get(1).unwrap();
            let smallest_area = first_smallest * second_smallest;
            let smallest_perimeter = 2 * (first_smallest + second_smallest);

            (
                paper + (l * w * 2) + (l * h * 2) + (h * w * 2) + smallest_area,
                ribbon + smallest_perimeter + (l * w * h),
            )
        })
}

const SAMPLE: &[u8] = b"2x3x4\n1x1x10\n";
const INPUT: &[u8] = include_bytes!("./inputs/day2.txt");

#[test]
fn both_parts() {
    let (sample_paper, sample_ribbon) = total_materials(SAMPLE);
    let (input_paper, input_ribbon) = total_materials(INPUT);

    // Part 1
    assert_eq!(101, sample_paper);
    assert_eq!(48, sample_ribbon);

    // Part 2
    assert_eq!(1588178, input_paper);
    assert_eq!(3783758, input_ribbon);
}
