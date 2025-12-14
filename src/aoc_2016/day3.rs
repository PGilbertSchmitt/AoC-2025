use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[ \n]")]
enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse::<usize>().unwrap())]
    Side(usize),
}

impl Token {
    fn value(self) -> usize {
        match self {
            Self::Side(val) => val,
        }
    }
}

fn parse_input_by_row(input: &str) -> Vec<(usize, usize, usize)> {
    let mut output = vec![];
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        output.push((
            token.unwrap().value(),
            lexer.next().unwrap().unwrap().value(),
            lexer.next().unwrap().unwrap().value(),
        ));
    }

    output
}

fn parse_input_by_col(input: &str) -> Vec<(usize, usize, usize)> {
    let mut output = vec![];
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        let a1 = token.unwrap().value();
        let a2 = lexer.next().unwrap().unwrap().value();
        let a3 = lexer.next().unwrap().unwrap().value();

        let b1 = lexer.next().unwrap().unwrap().value();
        let b2 = lexer.next().unwrap().unwrap().value();
        let b3 = lexer.next().unwrap().unwrap().value();

        let c1 = lexer.next().unwrap().unwrap().value();
        let c2 = lexer.next().unwrap().unwrap().value();
        let c3 = lexer.next().unwrap().unwrap().value();

        output.push((a1, b1, c1));
        output.push((a2, b2, c2));
        output.push((a3, b3, c3));
    }

    output
}

fn valid_triangle(triangle: (usize, usize, usize)) -> bool {
    let (x, y, z) = triangle;
    x < y + z && y < x + z && z < x + y
}

fn count_valid_triangles_by_row(input: &str) -> usize {
    let mut count = 0;
    for triangle in parse_input_by_row(input).into_iter() {
        if valid_triangle(triangle) {
            count += 1
        }
    }
    count
}

fn count_valid_triangles_by_col(input: &str) -> usize {
    let mut count = 0;
    for triangle in parse_input_by_col(input).into_iter() {
        if valid_triangle(triangle) {
            count += 1
        }
    }
    count
}

const INPUT: &'static str = include_str!("./inputs/day3.txt");

#[test]
fn part1() {
    assert_eq!(1032, count_valid_triangles_by_row(INPUT));
}

#[test]
fn part2() {
    assert_eq!(1838, count_valid_triangles_by_col(INPUT));
}
