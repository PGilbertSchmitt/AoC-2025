use std::collections::HashMap;

use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos(skip r"[ \n]")]
enum Token {
    #[token("NOT")]
    Not,
    #[token("AND")]
    And,
    #[token("OR")]
    Or,
    #[token("RSHIFT")]
    RShift,
    #[token("LSHIFT")]
    LShift,

    #[token("->")]
    Arrow,

    #[regex("[0-9]+", |lex| lex.slice().parse::<u16>().unwrap())]
    Integer(u16),

    #[regex("[a-z]+", |lex| lex.slice().to_owned())]
    Wire(String),
}

impl Token {
    fn is_not(&self) -> bool {
        match self {
            Self::Not => true,
            _ => false,
        }
    }

    fn take_source(self) -> Source {
        match self {
            Self::Integer(value) => Source::Signal(value),
            Self::Wire(value) => Source::Wire(value),
            _ => panic!("Attempted to pull missing `source value"),
        }
    }

    fn is_arrow(&self) -> bool {
        match self {
            Self::Arrow => true,
            _ => false,
        }
    }

    fn assert_arrow(self) {
        match self {
            Self::Arrow => (),
            _ => panic!("Not an arrow"),
        }
    }

    fn take_wire(self) -> String {
        match self {
            Self::Wire(value) => value,
            _ => panic!("Tried to take wire value from non-wire"),
        }
    }
}

enum Source {
    Wire(String),
    Signal(u16),
}

impl Source {
    fn take_signal(self) -> u16 {
        match self {
            Source::Signal(value) => value,
            Source::Wire(_) => panic!("Attempted to pull number from wire"),
        }
    }
}

enum Gate {
    Send(Source, String),
    Not(Source, String),
    And(Source, Source, String),
    Or(Source, Source, String),
    RShift(Source, u16, String),
    LShift(Source, u16, String),
}

impl Gate {
    fn output_label(&self) -> String {
        match self {
            Gate::Send(_, wire) => wire.to_owned(),
            Gate::Not(_, wire) => wire.to_owned(),
            Gate::Or(.., wire) => wire.to_owned(),
            Gate::And(.., wire) => wire.to_owned(),
            Gate::RShift(.., wire) => wire.to_owned(),
            Gate::LShift(.., wire) => wire.to_owned(),
        }
    }
}

// I would like to figure out how to write better quick parsers
fn read_line(lexer: &mut Lexer<'_, Token>) -> Option<Gate> {
    if let Some(token) = lexer.next() {
        let first = token.unwrap();
        if first.is_not() {
            Some(parse_not(lexer))
        } else {
            let operand_1 = first.take_source();
            let next = lexer.next().unwrap().unwrap();
            if next.is_arrow() {
                let output = lexer.next().unwrap().unwrap().take_wire();
                Some(Gate::Send(operand_1, output))
            } else {
                Some(parse_binop(lexer, operand_1, next))
            }
        }
    } else {
        None
    }
}

fn parse_not(lexer: &mut Lexer<'_, Token>) -> Gate {
    let operand = lexer.next().unwrap().unwrap().take_source();
    lexer.next().unwrap().unwrap().assert_arrow();
    let output = lexer.next().unwrap().unwrap().take_wire();
    Gate::Not(operand, output)
}

fn parse_binop(lexer: &mut Lexer<'_, Token>, operand_1: Source, op: Token) -> Gate {
    let operand_2 = lexer.next().unwrap().unwrap().take_source();
    lexer.next().unwrap().unwrap().assert_arrow();
    let output = lexer.next().unwrap().unwrap().take_wire();
    match op {
        Token::And => Gate::And(operand_1, operand_2, output),
        Token::Or => Gate::Or(operand_1, operand_2, output),
        Token::RShift => Gate::RShift(operand_1, operand_2.take_signal(), output),
        Token::LShift => Gate::LShift(operand_1, operand_2.take_signal(), output),
        _ => panic!("Expected op, found some other thing"),
    }
}

fn calculate_wire(
    label: &str,
    map: &HashMap<String, Gate>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(cached_value) = cache.get(label) {
        return *cached_value;
    }
    let cur_gate = map.get(label).unwrap();
    let result = match cur_gate {
        Gate::Send(source, _) => extract_value(&source, map, cache),
        Gate::Not(source, _) => extract_value(&source, map, cache) ^ u16::MAX,
        Gate::And(source_1, source_2, _) => {
            extract_value(source_1, map, cache) & extract_value(source_2, map, cache)
        }
        Gate::Or(source_1, source_2, _) => {
            extract_value(source_1, map, cache) | extract_value(source_2, map, cache)
        }
        Gate::RShift(source_1, offset, _) => extract_value(source_1, map, cache) >> offset,
        Gate::LShift(source_1, offset, _) => extract_value(source_1, map, cache) << offset,
    };
    cache.entry(label.to_owned()).or_insert(result);

    result
}

fn extract_value(
    source: &Source,
    map: &HashMap<String, Gate>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    match source {
        Source::Signal(value) => return *value,
        Source::Wire(label) => calculate_wire(&label, map, cache),
    }
}

fn calculate_all_wires(input: &str, b_override: Option<u16>) -> u16 {
    let mut lex = Token::lexer(input);

    let mut gate_map = HashMap::<String, Gate>::new();
    let mut gate_cache = HashMap::<String, u16>::new();
    if let Some(b_override) = b_override {
        gate_cache.insert(String::from("b"), b_override);
    }

    while let Some(gate) = read_line(&mut lex) {
        gate_map.insert(gate.output_label(), gate);
    }

    let gate_map = gate_map; // Remove mutability

    for (wire_label, _) in gate_map.iter() {
        calculate_wire(wire_label, &gate_map, &mut gate_cache);
    }

    *gate_cache.get("a").unwrap()
}

const INPUT: &'static str = include_str!("./inputs/day7.txt");

#[test]
fn both_parts() {
    let part_1_answer = calculate_all_wires(INPUT, None);
    assert_eq!(3176, part_1_answer);
    let part_2_answer = calculate_all_wires(INPUT, Some(part_1_answer));
    assert_eq!(14710, part_2_answer);
}
