use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[\n]")]
enum Token {
    #[token(r#"""#)]
    Quote,

    #[token(r#"\""#)]
    EscapedQuote,

    #[token(r#"\\"#)]
    Backslash,

    #[regex(r"\\x[a-fA-F0-9]{2}")]
    HexChar,

    #[regex("[a-z]+", |lex| lex.slice().len())]
    Raw(usize),
}

impl Token {
    fn code_size(&self) -> usize {
        match self {
            Self::Quote => 1,
            Self::EscapedQuote => 2,
            Self::Backslash => 2,
            Self::HexChar => 4,
            Self::Raw(val) => *val,
        }
    }

    fn data_size(&self) -> usize {
        match self {
            Self::Quote => 0,
            Self::EscapedQuote => 1,
            Self::Backslash => 1,
            Self::HexChar => 1,
            Self::Raw(val) => *val,
        }
    }

    fn new_size(&self) -> usize {
        match self {
            Self::Quote => 3,
            Self::EscapedQuote => 4,
            Self::Backslash => 4,
            Self::HexChar => 5,
            Self::Raw(val) => *val,
        }
    }
}

const INPUT: &'static str = include_str!("./inputs/day8.txt");

#[test]
fn part_1() {
    let mut diff = 0;
    for token_result in Token::lexer(INPUT) {
        let token = token_result.unwrap();
        diff += token.code_size() - token.data_size();
    }
    assert_eq!(1350, diff);
}

#[test]
fn part_2() {
    let mut diff = 0;
    for token_result in Token::lexer(INPUT) {
        let token = token_result.unwrap();
        diff += token.new_size() - token.code_size();
    }
    assert_eq!(2085, diff);
}
