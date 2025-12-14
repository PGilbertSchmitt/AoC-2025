use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[\n .]", skip r"Disc #\d+ has", skip r"positions; at time=0, it is at position")]
enum Token {
    #[regex(r"\d+", |lex| lex.slice().parse::<usize>().unwrap())]
    Number(usize),
}

impl Token {
    fn value(self) -> usize {
        match self {
            Self::Number(val) => val,
        }
    }
}

struct Disc {
    number: usize,
    num_positions: usize,
    cur_position: usize,
}

impl Disc {
    fn rotate(&mut self, time: usize) {
        self.cur_position += time;
        self.cur_position %= self.num_positions;
    }
}

fn parse_discs(input: &str) -> Vec<Disc> {
    let mut discs = Vec::new();
    let mut tokens = Token::lexer(input);
    while let Some(Ok(token)) = tokens.next() {
        let num_positions = token.value();
        let cur_position = tokens.next().unwrap().unwrap().value();
        discs.push(Disc {
            number: discs.len() + 1,
            num_positions,
            cur_position,
        });
    }
    discs
}

fn parse_with_extra_disc(input: &str) -> Vec<Disc> {
    let mut discs = parse_discs(input);
    discs.push(Disc {
        number: discs.len() + 1,
        num_positions: 11,
        cur_position: 0,
    });
    discs
}

fn rotate_discs(discs: &mut Vec<Disc>, time: usize) -> bool {
    let mut finished = true;

    for disc in discs {
        disc.rotate(time);
        if disc.cur_position != 0 {
            finished = false;
        }
    }

    finished
}

fn find_earliest_capsule_time(mut discs: Vec<Disc>) -> usize {
    let mut total_time = 0;
    let mut highest_num_positions = 0;
    let mut cur_pos_of_highest = 0;

    // First, pre-rotate based on the time that it takes the ball to fall to a disc.
    // This way we simply search for the time where all discs are at position 0. The
    // `number` attribute conveniently informs us how far to pre-rotate.
    for disc in &mut discs {
        disc.rotate(disc.number);
        if disc.num_positions > highest_num_positions {
            highest_num_positions = disc.num_positions;
            cur_pos_of_highest = disc.cur_position;
        }
    }

    // Now, we rotate all discs so that the disc with the highest number of positions
    // is at 0. This time is tracked.
    if cur_pos_of_highest > 0 {
        let mut finished_from_start = true;
        let added_time = highest_num_positions - cur_pos_of_highest;
        total_time += added_time;
        for disc in &mut discs {
            disc.rotate(added_time);
            if disc.cur_position != 0 {
                finished_from_start = false;
            }
        }
        // This is definitely not needed for input, but I'm a stickler for a solution
        // for ANY well formed input.
        if finished_from_start {
            return total_time;
        }
    }

    // Now the boring part
    loop {
        total_time += highest_num_positions;
        if rotate_discs(&mut discs, highest_num_positions) {
            break;
        }
    }

    // println!("Total time is {total_time}, we're working with {:?}", discs);

    total_time
}

const SAMPLE: &str = "
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
";

const INPUT: &str = "
Disc #1 has 7 positions; at time=0, it is at position 0.
Disc #2 has 13 positions; at time=0, it is at position 0.
Disc #3 has 3 positions; at time=0, it is at position 2.
Disc #4 has 5 positions; at time=0, it is at position 2.
Disc #5 has 17 positions; at time=0, it is at position 0.
Disc #6 has 19 positions; at time=0, it is at position 7.
";

#[test]
fn part_1() {
    assert_eq!(5, find_earliest_capsule_time(parse_discs(SAMPLE)));
    assert_eq!(121834, find_earliest_capsule_time(parse_discs(INPUT)));
}

#[test]
fn part_2() {
    // Amazingly, I did not need to use the Chinese remainder theorem here
    assert_eq!(
        85,
        find_earliest_capsule_time(parse_with_extra_disc(SAMPLE))
    );
    assert_eq!(
        3208099,
        find_earliest_capsule_time(parse_with_extra_disc(INPUT))
    );
}
