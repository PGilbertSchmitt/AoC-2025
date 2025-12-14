use fxhash::FxHashMap;
use logos::{Lexer, Logos};
use std::cmp::Ordering;

fn parse_checksum(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[1..slice.len() - 1].chars().collect()
}

#[derive(Logos)]
#[logos(skip r"[-\n]")]
enum Token {
    #[regex("[a-z]+", |lex| lex.slice().to_owned())]
    Segment(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<u32>().unwrap())]
    Id(u32),

    #[regex(r"\[[a-z]+\]", parse_checksum)]
    Checksum(String),
}

impl Token {
    fn segment_value(self) -> String {
        match self {
            Self::Segment(value) => value,
            _ => panic!("Can't extract segment value from non-segment"),
        }
    }
}

struct Room {
    id: u32,
    name_segments: Vec<String>,
    checksum: String,
}

fn parse_room(tokens: &mut Lexer<Token>) -> Option<Room> {
    let first_segment = tokens.next();
    let mut name_segments: Vec<String> = vec![];
    match first_segment {
        Some(tok) => name_segments.push(tok.unwrap().segment_value()),
        None => {
            return None;
        }
    }

    let mut id: Option<u32> = None;
    let checksum: String;

    loop {
        match tokens.next().unwrap().unwrap() {
            Token::Segment(value) => {
                name_segments.push(value);
            }
            Token::Id(value) => {
                id = Some(value);
            }
            Token::Checksum(value) => {
                checksum = value;
                break;
            }
        }
    }

    if id.is_none() {
        panic!("Failed to parse all components");
    }

    Some(Room {
        name_segments,
        id: id.unwrap(),
        checksum,
    })
}

fn parse_rooms(input: &str) -> Vec<Room> {
    let mut rooms = vec![];
    let mut tokens = Token::lexer(input).into_iter();
    loop {
        if let Some(room) = parse_room(&mut tokens) {
            rooms.push(room);
        } else {
            return rooms;
        }
    }
}

fn checksum_map(segments: &Vec<String>) -> FxHashMap<char, usize> {
    let mut char_map = FxHashMap::default();

    for seg in segments.into_iter() {
        for char in seg.chars() {
            char_map.entry(char).and_modify(|m| *m += 1).or_insert(1);
        }
    }

    char_map
}

#[derive(PartialEq, Eq, PartialOrd)]
struct HeatMapPair(char, usize);

impl Ord for HeatMapPair {
    fn cmp(&self, other: &Self) -> Ordering {
        // This ordering ensures that the count is ordered first (in reverse so that higher is sooner in the list)
        // and that if that fails, the character is sorted alphabetically (ie. normal order)
        other.1.cmp(&self.1).then(self.0.cmp(&other.0))
    }
}

fn validate_checksum(room: &Room) -> u32 {
    let heat_map = checksum_map(&room.name_segments);
    let mut pairs: Vec<HeatMapPair> = heat_map
        .into_iter()
        .map(|(ch, count)| HeatMapPair(ch, count))
        .collect::<Vec<_>>();
    pairs.sort_by(|a, b| a.cmp(b));

    if room.checksum == (&pairs[0..5]).iter().map(|v| v.0).collect::<String>() {
        room.id
    } else {
        0
    }
}

fn sum_of_valid_room_ids(input: &str) -> u32 {
    parse_rooms(input)
        .into_iter()
        .map(|room| validate_checksum(&room))
        .sum()
}

const LOWER_A: u32 = 'a' as u32;
fn apply_cypher(room: &Room) -> String {
    let increment = room.id as u32;
    let updated: Vec<String> = room
        .name_segments
        .iter()
        .map(|segment| {
            segment
                .chars()
                .map(|ch| ((((ch as u32) + increment - LOWER_A) % 26) + LOWER_A) as u8 as char)
                .collect::<String>()
        })
        .collect();
    return updated.join(" ");
}

fn find_northpole_objects_id(input: &str) -> Option<u32> {
    for room in parse_rooms(input).into_iter() {
        if validate_checksum(&room) > 0 && apply_cypher(&room) == "northpole object storage" {
            return Some(room.id);
        }
    }
    None
}

const SAMPLE: &'static str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
";
// qzmt-zixmtkozy-ivhz-343[zimth] => very encrypted name

const INPUT: &'static str = include_str!("./inputs/day4.txt");

#[test]
fn part_1() {
    assert_eq!(1514, sum_of_valid_room_ids(SAMPLE));
    assert_eq!(278221, sum_of_valid_room_ids(INPUT));
}

#[test]
fn part_2() {
    assert_eq!(Some(267), find_northpole_objects_id(INPUT));
}
