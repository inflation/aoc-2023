use color_eyre::eyre::eyre;
use winnow::{
    ascii::{digit1, space0, space1},
    combinator::{preceded, separated, separated_pair},
    BStr, PResult, Parser,
};

#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

fn parse_num(input: &mut &BStr) -> PResult<u32> {
    digit1.parse_to().parse_next(input)
}

fn parse_times(input: &mut &BStr) -> PResult<Vec<u32>> {
    preceded((b"Time:", space0), separated(1.., parse_num, space1)).parse_next(input)
}

fn parse_distances(input: &mut &BStr) -> PResult<Vec<u32>> {
    preceded((b"Distance:", space0), separated(1.., parse_num, space1)).parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Vec<Race>> {
    let (time, dist) = separated_pair(parse_times, "\n", parse_distances).parse_next(input)?;
    Ok(time
        .into_iter()
        .zip(dist)
        .map(|(time, dist)| Race { time, dist })
        .collect())
}

#[aoc_macro::main("day6")]
fn main() -> color_eyre::Result<u32> {
    let races = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut res = 1;
    for race in races {
        let mut count = 0;
        for t in 1..race.time {
            let d = t * (race.time - t);
            if d > race.dist {
                count += 1;
            }
        }
        res *= count;
    }

    Ok(res)
}
