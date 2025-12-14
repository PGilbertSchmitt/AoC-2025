use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"[ x\n]", skip r"by")]
enum Token {
    #[token("rect")]
    Rect,

    #[regex(r"\d+", |lex| lex.slice().parse::<usize>().unwrap())]
    Number(usize),

    #[token("rotate row y=")]
    RotateRow,

    #[token("rotate column x=")]
    RotateCol,
}

impl Token {
    fn get_number(self) -> usize {
        match self {
            Self::Number(val) => val,
            _ => panic!("NAN"),
        }
    }
}

enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

struct Screen([[bool; 50]; 6]);

impl Screen {
    fn new() -> Self {
        Self([[false; 50]; 6])
    }

    fn rect(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                self.0[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, count: usize) {
        self.0[row].rotate_right(count);
    }

    fn rotate_col(&mut self, col: usize, count: usize) {
        let mut column_values: Vec<bool> = self.0.iter().map(|row| row[col]).collect();
        column_values.rotate_right(count);
        for row in 0..6 {
            self.0[row][col] = column_values[row];
        }
    }

    fn _print(&self) {
        for row in self.0 {
            for col in row {
                if col {
                    print!("##");
                } else {
                    print!("  ");
                }
            }
            println!("");
        }
    }

    fn voltage(&self) -> usize {
        self.0
            .iter()
            .fold(0, |count, row| count + row.iter().filter(|x| **x).count())
    }
}

fn parse_instruction(tokens: &mut Lexer<Token>) -> Option<Instruction> {
    match tokens.next() {
        None => None,
        Some(Ok(Token::Rect)) => {
            let x = tokens.next().unwrap().unwrap();
            let y = tokens.next().unwrap().unwrap();
            return Some(Instruction::Rect(x.get_number(), y.get_number()));
        }
        Some(Ok(Token::RotateRow)) => {
            let row = tokens.next().unwrap().unwrap();
            let count = tokens.next().unwrap().unwrap();
            return Some(Instruction::RotateRow(row.get_number(), count.get_number()));
        }
        Some(Ok(Token::RotateCol)) => {
            let col = tokens.next().unwrap().unwrap();
            let count = tokens.next().unwrap().unwrap();
            return Some(Instruction::RotateCol(col.get_number(), count.get_number()));
        }
        _ => unreachable!(),
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut ins = vec![];
    let mut tokens = Token::lexer(input);

    while let Some(instruction) = parse_instruction(&mut tokens) {
        ins.push(instruction);
    }

    ins
}

fn execute_instructions(input: &str) -> Screen {
    let mut screen = Screen::new();
    for instruction in parse_instructions(input) {
        match instruction {
            Instruction::Rect(width, height) => screen.rect(width, height),
            Instruction::RotateRow(row, count) => screen.rotate_row(row, count),
            Instruction::RotateCol(col, count) => screen.rotate_col(col, count),
        }
    }

    screen
}

const INPUT: &'static str = include_str!("./inputs/day8.txt");

#[test]
fn both_parts() {
    let screen = execute_instructions(INPUT);
    assert_eq!(115, screen.voltage());

    // Should spell "EFEYKFRFIJ" in console
    // screen._jsprint();
}
