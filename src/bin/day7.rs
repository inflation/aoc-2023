use std::collections::HashMap;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::{alphanumeric1, digit1, newline},
    combinator::{separated, separated_pair},
    BStr, PResult, Parser,
};

fn parse_round<'s>(input: &mut &'s BStr) -> PResult<(&'s BStr, u32)> {
    separated_pair(alphanumeric1, " ", digit1.parse_to())
        .map(|(s, v)| (BStr::new(s), v))
        .parse_next(input)
}

fn parse_input<'s>(input: &mut &'s BStr) -> PResult<Vec<(&'s BStr, u32)>> {
    separated(1.., parse_round, newline).parse_next(input)
}

#[aoc_macro::main("day7")]
fn main() -> color_eyre::Result<u32> {
    let mut rounds: Vec<(&BStr, u32)> = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;
    rounds.sort_by_cached_key(|(k, _)| score(k));

    let res = rounds.into_iter().enumerate().fold(0, |acc, (i, (_, v))| {
        acc + (u32::try_from(i).unwrap() + 1) * v
    });

    Ok(res)
}

fn score_card(card: u8) -> u32 {
    match card {
        b'A' => 0xD,
        b'K' => 0xC,
        b'Q' => 0xB,
        b'T' => 0xA,
        x if x.is_ascii_digit() => u32::from(x - b'0'),
        b'J' => 1,
        _ => panic!("Invalid card"),
    }
}

fn score(hand: &BStr) -> (u8, u32) {
    let freq: HashMap<u8, u8> = hand.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_default() += 1;
        acc
    });

    let mut v: Vec<(u8, u8)> = freq.iter().map(|(k, v)| (*k, *v)).collect();
    v.sort_unstable_by_key(|(_, v)| std::cmp::Reverse(*v));
    let score = hand.iter().fold(0, |acc, x| acc * 16 + score_card(*x));

    let res = match v.len() {
        1 => (6, score),
        2 => {
            if freq.get(&b'J').is_some() {
                (6, score)
            } else if v[0].1 == 4 {
                (5, score)
            } else {
                (4, score)
            }
        }
        3 => {
            if v[0].1 == 3 {
                if freq.get(&b'J').is_some() {
                    (5, score)
                } else {
                    (3, score)
                }
            } else if let Some(j) = freq.get(&b'J') {
                if *j == 2 {
                    (5, score)
                } else {
                    (4, score)
                }
            } else {
                (2, score)
            }
        }
        4 => {
            if freq.get(&b'J').is_some() {
                (3, score)
            } else {
                (1, score)
            }
        }
        5 => {
            if freq.get(&b'J').is_some() {
                (1, score)
            } else {
                (0, score)
            }
        }
        _ => panic!("Invalid hand"),
    };
    res
}
