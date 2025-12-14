use logos::Logos;

#[derive(Logos)]
#[logos()]
enum Dir {
    #[token("U")]
    Up,
    #[token("R")]
    Right,
    #[token("D")]
    Down,
    #[token("L")]
    Left,

    #[token("\n")]
    Newline,
}

impl Dir {
    fn is_nl(&self) -> bool {
        match self {
            Self::Newline => true,
            _ => false,
        }
    }
}

struct KeypadPosition(u8, u8);

impl KeypadPosition {
    fn new() -> Self {
        Self(1, 1) // Number 5 is at position 1, 1 on the keypad
    }

    fn keypad_value(&self) -> u8 {
        self.0 + (self.1 * 3) + 1
    }

    fn next(&mut self, dir: Dir) {
        match dir {
            Dir::Left => {
                if self.0 > 0 {
                    self.0 -= 1;
                }
            }
            Dir::Right => {
                if self.0 < 2 {
                    self.0 += 1;
                }
            }
            Dir::Up => {
                if self.1 > 0 {
                    self.1 -= 1;
                }
            }
            Dir::Down => {
                if self.1 < 2 {
                    self.1 += 1;
                }
            }
            Dir::Newline => {} // Shouldn't be passed anyways
        }
    }
}

struct WorseKeypadPosition(usize, usize);

impl WorseKeypadPosition {
    fn new() -> Self {
        Self(0, 2) // Number 5 is at position 0, 2 on the keypad
    }

    // I couldn't think of a cleaner way to do this
    const KEYPAD: [[Option<char>; 5]; 5] = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ];

    fn keypad_value(&self) -> Option<char> {
        Self::KEYPAD[self.1][self.0]
    }

    fn next(&mut self, dir: Dir) {
        let old_value = (self.0, self.1);
        match dir {
            Dir::Left => {
                if self.0 > 0 {
                    self.0 -= 1;
                }
            }
            Dir::Right => {
                if self.0 < 4 {
                    self.0 += 1;
                }
            }
            Dir::Up => {
                if self.1 > 0 {
                    self.1 -= 1;
                }
            }
            Dir::Down => {
                if self.1 < 4 {
                    self.1 += 1;
                }
            }
            Dir::Newline => {} // Shouldn't be passed anyways
        }
        // Undo if we're now on a blank space
        if self.keypad_value().is_none() {
            self.0 = old_value.0;
            self.1 = old_value.1;
        }
    }
}

const SAMPLE: &'static str = "ULL
RRDDD
LURDL
UUUUD";

const INPUT: &'static str = include_str!("./inputs/day2.txt");

fn eval_code(input: &str) -> String {
    let mut code = vec![];

    let mut cur_key = KeypadPosition::new();
    let tokens = Dir::lexer(input.trim());
    for token in tokens {
        let dir = token.unwrap();
        if dir.is_nl() {
            code.push(cur_key.keypad_value());
        } else {
            cur_key.next(dir);
        }
    }
    code.push(cur_key.keypad_value());

    code.into_iter().map(|x| x.to_string()).collect()
}

fn eval_hard_code(input: &str) -> String {
    let mut code = vec![];

    let mut cur_key = WorseKeypadPosition::new();
    let tokens = Dir::lexer(input.trim());
    for token in tokens {
        let dir = token.unwrap();
        if dir.is_nl() {
            code.push(cur_key.keypad_value());
        } else {
            cur_key.next(dir);
        }
    }
    code.push(cur_key.keypad_value());

    code.into_iter().map(|x| x.unwrap().to_string()).collect()
}

#[test]
fn part_1() {
    assert_eq!("1985", eval_code(SAMPLE));
    assert_eq!("18843", eval_code(INPUT));
}

#[test]
fn part_2() {
    assert_eq!("5DB3", eval_hard_code(SAMPLE));
    assert_eq!("67BB9", eval_hard_code(INPUT));
}
