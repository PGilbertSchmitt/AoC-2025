use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"\n")]
enum Token {
    #[token("(")]
    OpenBrace,

    #[token(")")]
    CloseBrace,

    #[token("x")]
    Delim,

    #[regex(r"[0-9]", |lex| lex.slice().to_owned())]
    Number(String),

    #[regex(r"[A-Z]", |lex| lex.slice().to_owned())]
    Text(String),
}

impl Token {
    fn to_string(self) -> String {
        match self {
            Self::OpenBrace => String::from("("),
            Self::CloseBrace => String::from(")"),
            Self::Delim => String::from("x"),
            Self::Number(val) => val,
            Self::Text(val) => val,
        }
    }
}

fn parse_number(ts: &mut Lexer<Token>) -> (usize, usize) {
    let mut segment_length = 0;
    let mut number: usize = 0;
    while let Some(Ok(Token::Number(val))) = ts.next() {
        segment_length += 1;
        number = (number * 10) + val.parse::<usize>().unwrap();
    }
    (number, segment_length)
}

fn parse_marker(ts: &mut Lexer<Token>) -> String {
    let (char_count, _) = parse_number(ts);
    let (repeat_count, _) = parse_number(ts);
    let mut segment = Vec::with_capacity(char_count);
    for _ in 0..char_count {
        segment.push(ts.next().unwrap().unwrap().to_string())
    }
    segment.join("").repeat(repeat_count)
}

fn decompress(input: &str) -> String {
    let mut output_segments: Vec<String> = Vec::new();
    let mut tokens = Token::lexer(input);

    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::OpenBrace => {
                output_segments.push(parse_marker(&mut tokens));
            }
            other => output_segments.push(other.to_string()),
        }
    }

    output_segments.join("")
}

struct Segment {
    pure_segment_size: usize,
    markers: Vec<Marker>,
}

impl Segment {
    fn size(self) -> usize {
        self.pure_segment_size
            + self
                .markers
                .into_iter()
                .map(|marker| marker.size())
                .sum::<usize>()
    }
}

struct Marker {
    repeat: usize,
    segment: Segment,
}

impl Marker {
    fn size(self) -> usize {
        self.repeat * self.segment.size()
    }
}

fn parse_segment_v2(ts: &mut Lexer<Token>, length: usize) -> Segment {
    let mut remaining = length;
    let mut pure_segment_size: usize = 0;
    let mut markers: Vec<Marker> = Vec::new();

    while remaining > 0 {
        let token = ts.next();
        if token.is_none() {
            break;
        }
        let token = token.unwrap().unwrap();
        match token {
            Token::OpenBrace => {
                let (marker, compressed_size): (Marker, usize) = parse_marker_v2(ts);
                markers.push(marker);
                remaining -= compressed_size;
            }
            _ => {
                pure_segment_size += 1;
                remaining -= 1;
            }
        }
    }
    Segment {
        pure_segment_size,
        markers,
    }
}

fn parse_marker_v2(ts: &mut Lexer<Token>) -> (Marker, usize) {
    let (char_count, char_len) = parse_number(ts);
    let (repeat, repeat_len) = parse_number(ts);
    let segment = parse_segment_v2(ts, char_count);
    let compressed_length = char_count + char_len + repeat_len + 3; // 3 for the parens and 'x' chars
    (Marker { repeat, segment }, compressed_length)
}

fn decompress_v2(input: &str) -> Segment {
    let mut tokens = Token::lexer(input);
    parse_segment_v2(&mut tokens, input.len())
}

const SAMPLE_1: &str = "X(8x2)(3x3)ABCY";
const SAMPLE_2: &str = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
const SAMPLE_3: &str = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
const INPUT: &str = include_str!("./inputs/day9.txt");

#[test]
fn part1() {
    assert_eq!(18, decompress(SAMPLE_1).len());
    assert_eq!(152851, decompress(INPUT).len());
}

#[test]
fn part2() {
    assert_eq!(20, decompress_v2(SAMPLE_1).size());
    assert_eq!(445, decompress_v2(SAMPLE_2).size());
    assert_eq!(241920, decompress_v2(SAMPLE_3).size());
    assert_eq!(11797310782, decompress_v2(INPUT).size());
}
