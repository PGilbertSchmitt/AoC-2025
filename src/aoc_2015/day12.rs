use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"[^0-9\-\{\}]")]
enum Token {
    #[regex("-?[0-9]+", |lex| lex.slice().parse::<i32>().unwrap())]
    Number(i32),

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(r#":"red""#)]
    RedValue,
}

fn extract_values(lex: &mut Lexer<'_, Token>) -> i32 {
    let mut block_total = 0;

    let mut found_red = false;
    while let Some(token_res) = lex.next() {
        let token = token_res.unwrap();
        match token {
            Token::RedValue => {
                found_red = true;
            }
            Token::Number(val) => block_total += val,
            Token::OpenBrace => {
                // Wasted work if we've already found "red", but helps to keep track of depth.
                block_total += extract_values(lex);
            }
            Token::CloseBrace => {
                if found_red {
                    return 0;
                } else {
                    return block_total;
                };
            }
        }
    }

    block_total
}

const INPUT: &str = include_str!("./inputs/day12.txt");

#[test]
fn part_1() {
    let mut total = 0;
    for token in Token::lexer(INPUT) {
        if let Ok(Token::Number(val)) = token {
            total += val
        }
    }
    assert_eq!(111754, total);
}

#[test]
fn part_2() {
    let mut lex = Token::lexer(INPUT);
    assert_eq!(65402, extract_values(&mut lex));
}
