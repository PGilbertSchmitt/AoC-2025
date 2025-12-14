use logos::{Lexer, Logos};

#[derive(Logos)]
#[logos()]
enum Token {
    #[regex("[a-z]+", |lex| lex.slice().to_owned())]
    Supernet(String),

    #[regex(r"\[[a-z]+\]", |lex| {
        let slice = lex.slice();
        slice[1..slice.len()-1].to_owned()
    })]
    Hypernet(String),

    #[token("\n")]
    Newline,
}

struct Address {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

fn parse_address(tokens: &mut Lexer<Token>) -> Option<Address> {
    let mut addr = Address {
        supernets: Vec::new(),
        hypernets: Vec::new(),
    };

    for token in tokens {
        match token.unwrap() {
            Token::Supernet(value) => {
                addr.supernets.push(value);
            }
            Token::Hypernet(value) => {
                addr.hypernets.push(value);
            }
            Token::Newline => {
                break;
            }
        }
    }

    if addr.supernets.is_empty() || addr.hypernets.is_empty() {
        None
    } else {
        Some(addr)
    }
}

fn parse_addresses(input: &str) -> Vec<Address> {
    let mut addresses = vec![];
    let mut tokens = Token::lexer(input);

    while let Some(addr) = parse_address(&mut tokens) {
        addresses.push(addr);
    }

    addresses
}

fn has_abba(value: &[u8]) -> bool {
    for i in 0..value.len() - 3 {
        if value[i] != value[i + 1] && value[i] == value[i + 3] && value[i + 1] == value[i + 2] {
            return true;
        }
    }
    false
}

fn find_abas(value: &[u8], abas: &mut Vec<(u8, u8)>) {
    for i in 0..value.len() - 2 {
        if value[i] != value[i + 1] && value[i] == value[i + 2] {
            abas.push((value[i], value[i + 1]));
        }
    }
}

fn has_bab(value: &[u8], aba: (u8, u8)) -> bool {
    for i in 0..value.len() - 2 {
        if value[i] == aba.1 && value[i + 1] == aba.0 && value[i + 2] == aba.1 {
            return true;
        }
    }
    false
}

fn is_tls_address(address: &Address) -> bool {
    address
        .supernets
        .iter()
        .any(|snet| has_abba(snet.as_bytes()))
        && address
            .hypernets
            .iter()
            .all(|hnet| !has_abba(hnet.as_bytes()))
}

fn count_tls_addresses(input: &str) -> usize {
    parse_addresses(input)
        .iter()
        .filter(|addr| is_tls_address(addr))
        .count()
}

fn is_ssl_address(address: &Address) -> bool {
    let mut abas = vec![];
    for supernet in &address.supernets {
        find_abas(supernet.as_bytes(), &mut abas);
    }
    for hypernet in &address.hypernets {
        for aba in &abas {
            if has_bab(hypernet.as_bytes(), *aba) {
                return true;
            }
        }
    }
    false
}

fn count_ssl_addresses(input: &str) -> usize {
    parse_addresses(input)
        .iter()
        .filter(|addr| is_ssl_address(addr))
        .count()
}

const SAMPLE_1: &'static str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
";

const SAMPLE_2: &'static str = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
";

const INPUT: &'static str = include_str!("./inputs/day7.txt");

#[test]
fn part1() {
    assert_eq!(2, count_tls_addresses(SAMPLE_1));
    assert_eq!(105, count_tls_addresses(INPUT));
}

#[test]
fn part2() {
    assert_eq!(3, count_ssl_addresses(SAMPLE_2));
    assert_eq!(258, count_ssl_addresses(INPUT));
}
