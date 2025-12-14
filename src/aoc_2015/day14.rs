use logos::Logos;

#[derive(Logos)]
#[logos(skip "[A-Za-z\n ,/.]")]
enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().unwrap())]
    Number(usize),
}

impl Token {
    fn take_num(self) -> usize {
        match self {
            Self::Number(val) => val,
        }
    }
}

struct Reindeer {
    flight_speed: usize,
    flight_time: usize,
    rest_time: usize,
}

fn parse_reindeer(input: &str) -> Vec<Reindeer> {
    let mut lex = Token::lexer(input);

    let mut reindeer = Vec::new();
    while let Some(first_token) = lex.next() {
        let flight_speed = first_token.unwrap().take_num();
        let flight_time = lex.next().unwrap().unwrap().take_num();
        let rest_time = lex.next().unwrap().unwrap().take_num();

        reindeer.push(Reindeer {
            flight_speed,
            flight_time,
            rest_time,
        });
    }

    reindeer
}

fn get_distance(reindeer: &Reindeer, total_time: usize) -> usize {
    let segment_time = reindeer.flight_time + reindeer.rest_time;
    let whole_segments = total_time / segment_time;
    let remainder_time = total_time % segment_time;

    let remaining_flight_time = remainder_time.min(reindeer.flight_time);
    let total_flight_time = remaining_flight_time + (whole_segments * reindeer.flight_time);
    total_flight_time * reindeer.flight_speed
}

const INPUT: &str = include_str!("./inputs/day14.txt");

#[test]
fn part_1() {
    let mut max_dist = 0;
    for reindeer in parse_reindeer(INPUT) {
        max_dist = get_distance(&reindeer, 2503).max(max_dist);
    }
    assert_eq!(2655, max_dist);
}

#[test]
fn part_2() {
    let reindeer = parse_reindeer(INPUT);
    let mut scores: Vec<u16> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    for s in 0..2503 {
        let mut max_dist = 0;
        let mut winner_idx = 0;

        for (idx, reindeer) in reindeer.iter().enumerate() {
            let dist = get_distance(reindeer, s + 1);
            if dist > max_dist {
                max_dist = dist;
                winner_idx = idx;
            }
        }

        scores[winner_idx] += 1;
    }

    assert_eq!(Some(1059), scores.into_iter().max());
}
