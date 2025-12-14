use fxhash::FxHashMap;

// Too simple for a Logos lexer
fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .split("\n")
        .map(|segment| segment.to_owned())
        .collect()
}

fn create_heatmaps(messages: Vec<String>) -> Vec<FxHashMap<char, usize>> {
    let mut heatmaps: Vec<FxHashMap<char, usize>> = vec![];
    for _ in 0..messages.first().unwrap().len() {
        heatmaps.push(FxHashMap::default());
    }

    for message in messages {
        for (idx, ch) in message.char_indices() {
            heatmaps[idx]
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    heatmaps
}

fn decode_signal(input: &str, most: bool) -> String {
    let heatmaps = create_heatmaps(parse_input(input));
    heatmaps
        .into_iter()
        .map(|heatmap| {
            let mut entries = heatmap.into_iter().collect::<Vec<(char, usize)>>();
            entries.sort_by_key(|pair| pair.1);
            (if most {
                entries.last()
            } else {
                entries.first()
            })
            .unwrap()
            .0
        })
        .collect()
}

const SAMPLE: &'static str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

const INPUT: &'static str = include_str!("./inputs/day6.txt");

#[test]
fn part_1() {
    assert_eq!("easter", decode_signal(SAMPLE, true));
    assert_eq!("agmwzecr", decode_signal(INPUT, true));
}

#[test]
fn part_2() {
    // I actually found these answers first by accident before realizing I reversed the sort order
    assert_eq!("advent", decode_signal(SAMPLE, false));
    assert_eq!("owlaxqvq", decode_signal(INPUT, false));
}
