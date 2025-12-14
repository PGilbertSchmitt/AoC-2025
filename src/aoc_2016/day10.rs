use std::collections::VecDeque;

use fxhash::FxHashMap;
use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[\n ]", skip "gives low to", skip "and high to", skip "goes to")]
enum Token {
    #[regex(r"bot [0-9]+", |lex| lex.slice()[4..].parse::<usize>().unwrap())]
    Bot(usize),

    #[regex(r"output [0-9]+", |lex| lex.slice()[7..].parse::<usize>().unwrap())]
    Output(usize),

    #[regex(r"value [0-9]+", |lex| lex.slice()[6..].parse::<usize>().unwrap())]
    Value(usize),
}

impl Token {
    fn to_value(self) -> usize {
        match self {
            Self::Bot(val) | Self::Output(val) | Self::Value(val) => val,
        }
    }

    fn is_input(&self) -> bool {
        match self {
            &Self::Value(_) => true,
            _ => false,
        }
    }

    fn to_destination(self) -> Dest {
        match self {
            Self::Bot(val) => Dest::Bot(val),
            Self::Output(val) => Dest::Output(val),
            _ => panic!("Attempted to treat value op as an output"),
        }
    }
}

// #[derive(Clone, Copy)]
enum Dest {
    Bot(usize),
    Output(usize),
}

struct Transfer {
    low: Dest,
    high: Dest,
}

struct ValueIns {
    value: usize,
    bot: usize,
}

fn parse_instructions(input: &str) -> (Vec<ValueIns>, FxHashMap<usize, Transfer>, Vec<usize>) {
    let mut values = Vec::new();
    let mut transfers = FxHashMap::default();
    let mut all_ids = Vec::new();
    let mut tokens = Token::lexer(input);
    while let Some(Ok(head)) = tokens.next() {
        let is_input = head.is_input();
        let input_value = head.to_value();
        if is_input {
            let bot = tokens.next().unwrap().unwrap().to_value();
            values.push(ValueIns {
                value: input_value,
                bot,
            });
        } else {
            all_ids.push(input_value);
            let low = tokens.next().unwrap().unwrap().to_destination();
            let high = tokens.next().unwrap().unwrap().to_destination();
            transfers.insert(input_value, Transfer { low, high });
        }
    }
    (values, transfers, all_ids)
}

enum BotState {
    Empty,
    One(usize),
    Both(usize, usize),
}

impl BotState {
    fn add_value(&mut self, new_value: usize) -> bool {
        let mut resolved = false;
        *self = match self {
            &mut Self::Empty => Self::One(new_value),
            &mut Self::One(high_value) => {
                resolved = true;
                Self::Both(new_value, high_value)
            }
            &mut Self::Both(..) => panic!("Bot already has 2 values"),
        };
        resolved
    }

    fn get_low_high(&self) -> (usize, usize) {
        match self {
            &Self::Both(a, b) => {
                if a < b {
                    (a, b)
                } else {
                    (b, a)
                }
            }
            _ => panic!("Expected bot to have both values"),
        }
    }
}

fn initialize_bot_states(
    all_ids: Vec<usize>,
    values: Vec<ValueIns>,
) -> (FxHashMap<usize, BotState>, VecDeque<usize>) {
    let mut bot_map = FxHashMap::default();
    for id in all_ids {
        bot_map.insert(id, BotState::Empty);
    }

    let mut resolved_bots = VecDeque::new();
    for ValueIns { value, bot } in values {
        // bot_map.entry(value).and_modify()
        let state = bot_map.get_mut(&bot).unwrap();
        if state.add_value(value) {
            resolved_bots.push_back(bot);
        }
    }

    (bot_map, resolved_bots)
}

fn process_bot_commands(input: &str) -> (FxHashMap<usize, usize>, FxHashMap<usize, BotState>) {
    let (values, transfers, all_ids) = parse_instructions(input);

    // Initialize bot states
    let (mut bot_map, mut resolved) = initialize_bot_states(all_ids, values);

    let mut output_map: FxHashMap<usize, usize> = FxHashMap::default();

    while let Some(bot_id) = resolved.pop_front() {
        let (low_value, high_value) = bot_map.get_mut(&bot_id).unwrap().get_low_high();
        let Transfer { low, high } = transfers.get(&bot_id).unwrap();
        match low {
            Dest::Bot(low_bot) => {
                let state = bot_map.get_mut(low_bot).unwrap();
                if state.add_value(low_value) {
                    resolved.push_back(*low_bot);
                }
            }
            Dest::Output(output) => {
                output_map.insert(*output, low_value);
            }
        }
        match high {
            Dest::Bot(high_bot) => {
                let state = bot_map.get_mut(high_bot).unwrap();
                if state.add_value(high_value) {
                    resolved.push_back(*high_bot);
                }
            }
            Dest::Output(output) => {
                output_map.insert(*output, high_value);
            }
        }
    }

    (output_map, bot_map)
}

fn find_part_1_bot(states: &FxHashMap<usize, BotState>) -> Option<usize> {
    for state in states {
        let (low, high) = state.1.get_low_high();
        if low == 17 && high == 61 {
            return Some(*state.0);
        }
    }
    None
}

const INPUT: &str = include_str!("./inputs/day10.txt");

#[test]
fn both_parts() {
    let (outputs, states) = process_bot_commands(INPUT);
    assert_eq!(Some(56), find_part_1_bot(&states));

    let prod = outputs.get(&0).unwrap_or(&0)
        * outputs.get(&1).unwrap_or(&0)
        * outputs.get(&2).unwrap_or(&0);
    assert_eq!(7847, prod);
}
