use core::str;
use logos::{Lexer, Logos};
use std::{collections::HashSet, fmt::Display};

use crate::util::Pair;

#[derive(Logos)]
#[logos(skip r"[ ,\n]", skip r"through")]
enum Token {
    // Actions
    #[token("turn off")]
    TurnOff,

    #[token("turn on")]
    TurnOn,

    #[token("toggle")]
    Toggle,

    // Other
    #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
    Integer(isize),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TurnOff => f.write_str("Turn Off"),
            Self::TurnOn => f.write_str("Turn On"),
            Self::Toggle => f.write_str("Toggle"),
            Self::Integer(value) => f.write_fmt(format_args!("Integer({value})")),
        }
    }
}

enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

impl From<Token> for Action {
    fn from(value: Token) -> Self {
        match value {
            Token::TurnOff => Action::TurnOff,
            Token::TurnOn => Action::TurnOn,
            Token::Toggle => Action::Toggle,
            _ => panic!("Attempted to read action from number, parser was desynced"),
        }
    }
}

impl From<Token> for isize {
    fn from(value: Token) -> Self {
        match value {
            Token::Integer(value) => value,
            _ => panic!("Attempted to read number from action, parser was desynced"),
        }
    }
}

struct Command {
    action: Action,
    start: Pair,
    end: Pair,
}

fn read_line(lexer: &mut Lexer<'_, Token>) -> Option<Command> {
    if let Some(action) = lexer.next() {
        Some(Command {
            action: action.unwrap().into(),
            start: Pair(
                lexer.next().unwrap().unwrap().into(),
                lexer.next().unwrap().unwrap().into(),
            ),
            end: Pair(
                lexer.next().unwrap().unwrap().into(),
                lexer.next().unwrap().unwrap().into(),
            ),
        })
    } else {
        None
    }
}

fn parse_commands(input: &'static str) -> Vec<Command> {
    let mut lex = Token::lexer(input);
    let mut commands = Vec::<Command>::new();
    while let Some(command) = read_line(&mut lex) {
        commands.push(command);
    }
    commands
}

fn light(value: bool) -> isize {
    if value { 1 } else { 0 }
}

// Iterate through the commands in reverse order until we either reach a terminal action
// (turn-on or turn-off) or the end of the list (which means off by default). For every
// toggle we reach, we invert the toggle state, and when we return, we invert if the toggle
// is active. Doing this in reverse order means we have terminal commands that can prevent
// use from having to iterate through all commands.
fn is_lit(commands: &Vec<Command>, x: isize, y: isize) -> isize {
    let mut toggled = false;
    for Command { action, start, end } in commands.iter().rev() {
        if start.0 <= x && x <= end.0 && start.1 <= y && y <= end.1 {
            match action {
                Action::TurnOn => return light(true != toggled),
                Action::TurnOff => return light(false != toggled),
                Action::Toggle => {
                    toggled = !toggled;
                }
            }
        }
    }
    light(false != toggled)
}

// Since we need the full context of all commands that a pixel may go through, we can't
// terminate early like with part 1. Still, it's just quick maffs.
fn brightness_level(commands: &Vec<Command>, x: isize, y: isize) -> isize {
    let mut level: isize = 0;
    for Command { action, start, end } in commands.iter() {
        if start.0 <= x && x <= end.0 && start.1 <= y && y <= end.1 {
            match action {
                Action::TurnOn => level += 1,
                Action::TurnOff => level = (level - 1).max(0),
                Action::Toggle => level += 2,
            }
        }
    }
    level
}

fn get_boundaries<'a, I>(commands: I) -> (Vec<isize>, Vec<isize>)
where
    I: Iterator<Item = &'a Command>,
{
    let mut x_boundaries = HashSet::<isize>::new();
    let mut y_boundaries = HashSet::<isize>::new();
    x_boundaries.insert(0);
    y_boundaries.insert(0);

    for command in commands {
        x_boundaries.insert(command.start.0);
        x_boundaries.insert(command.end.0 + 1);
        y_boundaries.insert(command.start.1);
        y_boundaries.insert(command.end.1 + 1);
    }

    let mut x_boundaries: Vec<isize> = x_boundaries.into_iter().collect();
    let mut y_boundaries: Vec<isize> = y_boundaries.into_iter().collect();
    x_boundaries.sort();
    y_boundaries.sort();

    (x_boundaries, y_boundaries)
}

fn get_lit_lights(commands: &Vec<Command>, f: fn(&Vec<Command>, isize, isize) -> isize) -> isize {
    let (x_boundaries, y_boundaries) = get_boundaries(commands.iter());

    let mut on_count: isize = 0;

    let mut xs = x_boundaries.iter().peekable();
    while let Some(cur_x) = xs.next() {
        let next_x = xs.peek().unwrap_or(&&1000);
        let x_len = **next_x - *cur_x;

        let mut ys = y_boundaries.iter().peekable();
        while let Some(cur_y) = ys.next() {
            let next_y = ys.peek().unwrap_or(&&1000);
            let y_len = **next_y - *cur_y;
            let block_size = x_len * y_len;

            on_count += f(commands, *cur_x, *cur_y) * block_size;
        }
    }

    on_count
}

const INPUT: &'static str = include_str!("./inputs/day6.txt");

// This was way faster than I was expecting
#[test]
fn both_parts() {
    let commands = parse_commands(INPUT);
    assert_eq!(400_410, get_lit_lights(&commands, is_lit));
    assert_eq!(15_343_601, get_lit_lights(&commands, brightness_level));
}
