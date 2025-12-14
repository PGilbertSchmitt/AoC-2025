use logos::Logos;

const INPUT: &'static str = include_str!("./inputs/day23.txt");

#[derive(Logos)]
#[logos(skip r"[\n ,]")]
enum Token {
    #[token("hlf")]
    Half,

    #[token("tpl")]
    Triple,

    #[token("inc")]
    Increment,

    #[token("jmp")]
    Jump,

    #[token("jie")]
    JumpIfEven,

    #[token("jio")]
    JumpIfOne,

    #[regex("[ab]", |lex| lex.slice().chars().nth(0).unwrap())]
    Register(char),

    #[regex("[+-][0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Offset(i64),
}

impl Token {
    fn to_register(self) -> Register {
        match self {
            Self::Register(ch) => {
                if ch == 'a' {
                    Register::A
                } else {
                    Register::B
                }
            }
            _ => unreachable!(),
        }
    }

    fn to_offset(self) -> i64 {
        match self {
            Self::Offset(offset) => offset,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Register {
    A,
    B,
}

enum Ins {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i64),
    JumpIfEven(Register, i64),
    JumpIfOne(Register, i64),
}

type Instructions = Vec<Ins>;

fn create(input: &str) -> Instructions {
    let mut instructions = Vec::new();
    let mut tokens = Token::lexer(input);
    while let Some(Ok(token)) = tokens.next() {
        match token {
            Token::Half => {
                let reg = tokens.next().unwrap().unwrap().to_register();
                instructions.push(Ins::Half(reg));
            }
            Token::Triple => {
                let reg = tokens.next().unwrap().unwrap().to_register();
                instructions.push(Ins::Triple(reg));
            }
            Token::Increment => {
                let reg = tokens.next().unwrap().unwrap().to_register();
                instructions.push(Ins::Increment(reg));
            }
            Token::Jump => {
                let offset = tokens.next().unwrap().unwrap().to_offset();
                instructions.push(Ins::Jump(offset));
            }
            Token::JumpIfEven => {
                let reg = tokens.next().unwrap().unwrap().to_register();
                let offset = tokens.next().unwrap().unwrap().to_offset();
                instructions.push(Ins::JumpIfEven(reg, offset));
            }
            Token::JumpIfOne => {
                let reg = tokens.next().unwrap().unwrap().to_register();
                let offset = tokens.next().unwrap().unwrap().to_offset();
                instructions.push(Ins::JumpIfOne(reg, offset));
            }
            _ => unreachable!(),
        }
    }

    instructions
}

fn exec(instructions: &Instructions, init_a: i32) -> i32 {
    let mut ptr: usize = 0;
    let mut a = init_a;
    let mut b = 0;
    while let Some(ins) = instructions.get(ptr) {
        let mut iptr = ptr as i64;
        match ins {
            Ins::Half(Register::A) => {
                a /= 2;
                iptr += 1;
            }
            Ins::Half(Register::B) => {
                b /= 2;
                iptr += 1;
            }
            Ins::Triple(Register::A) => {
                a *= 3;
                iptr += 1;
            }
            Ins::Triple(Register::B) => {
                b *= 3;
                iptr += 1;
            }
            Ins::Increment(Register::A) => {
                a += 1;
                iptr += 1;
            }
            Ins::Increment(Register::B) => {
                b += 1;
                iptr += 1;
            }
            Ins::Jump(rel) => {
                iptr += rel;
            }
            Ins::JumpIfEven(Register::A, rel) => {
                iptr += if a % 2 == 0 { *rel } else { 1 };
            }
            Ins::JumpIfEven(Register::B, rel) => {
                iptr += if b % 2 == 0 { *rel } else { 1 };
            }
            Ins::JumpIfOne(Register::A, rel) => {
                iptr += if a == 1 { *rel } else { 1 };
            }
            Ins::JumpIfOne(Register::B, rel) => {
                iptr += if b == 1 { *rel } else { 1 };
            }
        }
        if iptr.is_negative() {
            break;
        } else {
            ptr = iptr as usize;
        }
    }

    b
}

#[test]
fn both_parts() {
    let instructions = create(INPUT);
    assert_eq!(255, exec(&instructions, 0));
    assert_eq!(334, exec(&instructions, 1));
}
