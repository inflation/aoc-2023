use std::collections::HashSet;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::{digit1, space0, space1},
    combinator::{delimited, preceded, separated, separated_pair},
    error::ContextError,
    token::take_until0,
    PResult, Parser,
};

#[derive(Debug)]
struct Card {
    win: HashSet<u32>,
    num: HashSet<u32>,
}

fn parse_num<'s>(input: &mut &'s [u8]) -> PResult<u32, ContextError<&'s str>> {
    digit1.parse_to().parse_next(input)
}

fn parse_numbers<'s>(input: &mut &'s [u8]) -> PResult<Vec<u32>, ContextError<&'s str>> {
    delimited(space0, separated(1.., parse_num, space1), space0).parse_next(input)
}

fn parse_card<'s>(input: &mut &'s [u8]) -> PResult<Card, ContextError<&'s str>> {
    let (win, num) = preceded(
        (take_until0(":"), ":"),
        separated_pair(parse_numbers, b"|", parse_numbers),
    )
    .parse_next(input)?;
    Ok(Card {
        win: HashSet::from_iter(win),
        num: HashSet::from_iter(num),
    })
}

fn parse_input<'s>(input: &mut &'s [u8]) -> PResult<Vec<Card>, ContextError<&'s str>> {
    separated(1.., parse_card, b"\n").parse_next(input)
}

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read(std::env::var("INPUT")?)?;

    let cards = parse_input.parse(input.as_ref()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut coefficient: Vec<u32> = cards.iter().map(|_| 1).collect();
    for (i, card) in cards.into_iter().enumerate() {
        let n = card.win.intersection(&card.num).count();
        for j in i + 1..i + 1 + n {
            if j >= coefficient.len() {
                break;
            }
            coefficient[j] += coefficient[i];
        }
    }

    let sum: u32 = coefficient.into_iter().sum();

    println!("ANSWER: {sum}");
    Ok(())
}
