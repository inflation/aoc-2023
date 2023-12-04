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

#[aoc_macro::main("day4")]
fn main() -> color_eyre::Result<u32> {
    let cards = parse_input.parse(input.as_ref()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut sum = 0;
    for card in cards {
        let n = card.win.intersection(&card.num).count();
        if n > 0 {
            sum += 2_u32.pow(u32::try_from(n - 1).unwrap());
        }
    }

    Ok(sum)
}
