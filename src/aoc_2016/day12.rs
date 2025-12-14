use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"[\n ]")]
enum Token {
    #[token("cpy")]
    Copy,

    #[token("inc")]
    Increment,

    #[token("dec")]
    Decrement,

    #[token("jnz")]
    JumpNotZero,

    #[regex("[a-z]", |lex| lex.slice().chars().nth(0))]
    Register(char),

    #[regex("-?[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Value(isize),
}

enum Operand {
    Register(usize),
    Value(isize),
}

impl Operand {
    fn get_reg(self) -> usize {
        match self {
            Operand::Register(reg) => reg,
            _ => panic!("Not a register"),
        }
    }

    fn to_value(&self, registers: &Registers) -> isize {
        match self {
            Self::Value(val) => *val,
            Self::Register(reg) => registers[*reg],
        }
    }
}

impl From<Token> for Operand {
    fn from(value: Token) -> Self {
        match value {
            Token::Value(val) => Operand::Value(val),
            Token::Register(reg) => Operand::Register(match reg {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                _ => unreachable!(),
            }),
            _ => panic!("Can't convert Token to Datum"),
        }
    }
}

enum Instruction {
    Cpy(Operand, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Operand, Operand),
}

fn get_operand(tokens: &mut Lexer<Token>) -> Operand {
    Operand::from(tokens.next().unwrap().unwrap())
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut tokens = Token::lexer(input);

    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::Copy => {
                let x = get_operand(&mut tokens);
                let y = get_operand(&mut tokens).get_reg();
                instructions.push(Instruction::Cpy(x, y));
            }
            Token::Increment => {
                let x = get_operand(&mut tokens).get_reg();
                instructions.push(Instruction::Inc(x));
            }
            Token::Decrement => {
                let x = get_operand(&mut tokens).get_reg();
                instructions.push(Instruction::Dec(x));
            }
            Token::JumpNotZero => {
                let x = get_operand(&mut tokens);
                let y = get_operand(&mut tokens);
                instructions.push(Instruction::Jnz(x, y));
            }
            _ => panic!("Unexpected token"),
        }
    }

    instructions
}

type Registers = [isize; 4];

fn execute(ins: &Vec<Instruction>, register_c: isize) -> isize {
    let mut ptr = 0;
    let mut state: Registers = [0, 0, register_c, 0];
    let len = ins.len();

    while ptr < len {
        let cur_ins = &ins[ptr];

        match cur_ins {
            Instruction::Cpy(x, y) => {
                let src = x.to_value(&state);
                state[*y] = src;
                ptr += 1;
            }
            Instruction::Inc(x) => {
                state[*x] += 1;
                ptr += 1;
            }
            Instruction::Dec(x) => {
                state[*x] -= 1;
                ptr += 1;
            }
            Instruction::Jnz(x, y) => {
                if x.to_value(&state) != 0 {
                    let delta = y.to_value(&state);
                    if delta.is_negative() {
                        let delta_pos = delta.abs() as usize;
                        if delta_pos > ptr {
                            panic!("Can't make ptr negative");
                        }
                        ptr -= delta_pos;
                    } else {
                        ptr += delta as usize;
                    }
                } else {
                    ptr += 1;
                }
            }
        }
    }

    state[0]
}

const SAMPLE: &str = "
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

const INPUT: &str = include_str!("./inputs/day12.txt");

#[test]
fn both_parts() {
    let sample_ins = parse_instructions(SAMPLE);
    let input_ins = parse_instructions(INPUT);

    assert_eq!(42, execute(&sample_ins, 0));
    assert_eq!(317993, execute(&input_ins, 0));
    assert_eq!(9227647, execute(&input_ins, 1));
}
