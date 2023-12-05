use std::{collections::HashMap, ops::Range};

use color_eyre::eyre::eyre;
use winnow::{
    ascii::digit1,
    combinator::{delimited, preceded, separated},
    token::take_until1,
    BStr, PResult, Parser,
};

type Map = HashMap<Range<u64>, Range<u64>>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

fn parse_numbers(input: &mut &BStr) -> PResult<Vec<u64>> {
    separated(1.., digit1.parse_to::<u64>(), b" ").parse_next(input)
}

fn parse_map(input: &mut &BStr) -> PResult<Map> {
    preceded(
        (take_until1("\n"), b"\n"),
        separated(1.., parse_numbers, "\n").map(|v: Vec<_>| {
            v.into_iter()
                .map(|ns| (ns[1]..ns[1] + ns[2], ns[0]..ns[0] + ns[2]))
                .collect()
        }),
    )
    .parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Almanac> {
    let seeds = delimited(b"seeds: ", parse_numbers, b"\n\n").parse_next(input)?;
    let maps = separated(1.., parse_map, b"\n\n").parse_next(input)?;

    Ok(Almanac { seeds, maps })
}

#[aoc_macro::main("day5")]
fn main() -> color_eyre::Result<u64> {
    let almanac = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut min = u64::MAX;
    for seed in &almanac.seeds {
        let mut loc = *seed;
        for map in &almanac.maps {
            loc = map
                .iter()
                .find(|(src, _)| src.contains(&loc))
                .map_or(loc, |(src, dst)| dst.start + loc - src.start);
        }
        eprintln!("Location: {loc}");
        if loc < min {
            min = loc;
        }
    }

    Ok(min)
}
