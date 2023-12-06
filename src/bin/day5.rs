use std::ops::Range;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::digit1,
    combinator::{delimited, preceded, separated, separated_pair},
    token::take_until1,
    BStr, PResult, Parser,
};

type Map = Vec<(u64, u64, u64)>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<Map>,
}

fn parse_num(input: &mut &BStr) -> PResult<u64> {
    digit1.parse_to().parse_next(input)
}

fn parse_numbers(input: &mut &BStr) -> PResult<(u64, u64, u64)> {
    (parse_num, b" ", parse_num, b" ", parse_num)
        .map(|(a, _, b, _, c)| (a, b, c))
        .parse_next(input)
}

fn parse_seeds(input: &mut &BStr) -> PResult<Vec<Range<u64>>> {
    separated(
        1..,
        separated_pair(parse_num, b" ", parse_num).map(|(start, len)| start..start + len),
        b" ",
    )
    .parse_next(input)
}

fn parse_map(input: &mut &BStr) -> PResult<Map> {
    preceded(
        (take_until1("\n"), b"\n"),
        separated(1.., parse_numbers, "\n").map(|v: Vec<_>| v.into_iter().collect()),
    )
    .parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Almanac> {
    let seeds = delimited(b"seeds: ", parse_seeds, b"\n\n").parse_next(input)?;
    let maps = separated(1.., parse_map, b"\n\n").parse_next(input)?;

    Ok(Almanac { seeds, maps })
}

#[aoc_macro::main("day5")]
fn main() -> color_eyre::Result<u64> {
    let Almanac { seeds, maps } = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    eprintln!("Parsed");

    let mut min = u64::MAX;
    for range in seeds {
        let mut r = vec![(range.start, range.end)];
        for f in &maps {
            r = apply_range(r, f);
        }

        if let Some(l) = r.iter().min_by_key(|m| m.0) {
            min = l.0.min(min);
        }
    }

    Ok(min)
}

fn apply_range(mut r: Vec<(u64, u64)>, map: &Map) -> Vec<(u64, u64)> {
    let mut a = vec![];

    for (dst, src, len) in map {
        let src_end = src + len;
        let mut nr = vec![];

        while let Some((st, ed)) = r.pop() {
            let before = (st, ed.min(*src));
            let inter = (st.max(*src), src_end.min(ed));
            let after = (src_end.max(st), ed);

            if before.1 > before.0 {
                nr.push(before);
            }
            if inter.1 > inter.0 {
                a.push((dst + inter.0 - src, inter.1 - src + dst));
            }
            if after.1 > after.0 {
                nr.push(after);
            }
        }

        r = nr;
    }

    a.append(&mut r);
    a
}
