use logos::Logos;

#[derive(Logos)]
#[logos(skip "[ :\n,]", skip "Sue")]
enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse::<u16>().unwrap())]
    Count(u16),

    #[regex("[a-z]+", |lex| lex.slice().to_owned())]
    Compound(String),
}

impl Token {
    fn to_compound(self, count: u16) -> Compound {
        match self {
            Self::Compound(s) => Compound::from_string(s, count),
            _ => panic!("This is a number, numb-nuts!"),
        }
    }

    fn take_count(self) -> u16 {
        match self {
            Self::Count(v) => v,
            _ => panic!("This is not a number, silly-pants!"),
        }
    }
}

enum Compound {
    Children(u16),
    Cats(u16),
    Samoyeds(u16),
    Pomeranians(u16),
    Akitas(u16),
    Vizslas(u16),
    Goldfish(u16),
    Trees(u16),
    Cars(u16),
    Perfumes(u16),
}

impl Compound {
    fn from_string(value: String, count: u16) -> Self {
        match value.as_ref() {
            "children" => Self::Children(count),
            "cats" => Self::Cats(count),
            "samoyeds" => Self::Samoyeds(count),
            "pomeranians" => Self::Pomeranians(count),
            "akitas" => Self::Akitas(count),
            "vizslas" => Self::Vizslas(count),
            "goldfish" => Self::Goldfish(count),
            "trees" => Self::Trees(count),
            "cars" => Self::Cars(count),
            "perfumes" => Self::Perfumes(count),
            _ => panic!("'{value}' is not a known compount"),
        }
    }

    fn matches_exact(&self) -> bool {
        match self {
            Self::Children(v) => *v == 3,
            Self::Cats(v) => *v == 7,
            Self::Samoyeds(v) => *v == 2,
            Self::Pomeranians(v) => *v == 3,
            Self::Akitas(v) => *v == 0,
            Self::Vizslas(v) => *v == 0,
            Self::Goldfish(v) => *v == 5,
            Self::Trees(v) => *v == 3,
            Self::Cars(v) => *v == 2,
            Self::Perfumes(v) => *v == 1,
        }
    }

    fn matches_with_ranges(&self) -> bool {
        match self {
            Self::Children(v) => *v == 3,
            Self::Cats(v) => *v > 7,
            Self::Samoyeds(v) => *v == 2,
            Self::Pomeranians(v) => *v < 3,
            Self::Akitas(v) => *v == 0,
            Self::Vizslas(v) => *v == 0,
            Self::Goldfish(v) => *v < 5,
            Self::Trees(v) => *v > 3,
            Self::Cars(v) => *v == 2,
            Self::Perfumes(v) => *v == 1,
        }
    }
}

// What a silly name for a struct
// struct AuntSue {
//     aunt_id: u16,
//     known_compounds: [Compound; 3],
// }

fn find_valid_aunt(input: &str, matcher: fn(&Compound) -> bool) -> Option<u16> {
    let mut lex = Token::lexer(input);

    while let Some(tok_res) = lex.next() {
        let aunt_id = tok_res.unwrap().take_count();

        // Every aunt has 3 pairs of compound/count
        let mut still_valid = true;
        for _ in 0..3 {
            let compound_token = lex.next().unwrap().unwrap();
            let count = lex.next().unwrap().unwrap().take_count();

            if still_valid && !matcher(&compound_token.to_compound(count)) {
                still_valid = false;
            }
        }

        if still_valid {
            return Some(aunt_id);
        }
    }

    None
}

const INPUT: &str = include_str!("./inputs/day16.txt");

#[test]
fn part_1() {
    assert_eq!(Some(40), find_valid_aunt(INPUT, Compound::matches_exact));
    assert_eq!(
        Some(241),
        find_valid_aunt(INPUT, Compound::matches_with_ranges)
    );
}
