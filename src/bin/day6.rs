use color_eyre::eyre::eyre;
use winnow::{
    ascii::space0,
    combinator::{preceded, separated_pair},
    token::take_till,
    BStr, PResult, Parser,
};

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}

fn parse_num(input: &mut &BStr) -> PResult<u64> {
    take_till(1.., |c: u8| !(c.is_ascii_digit() || c == b' '))
        .map(|s: &[u8]| {
            s.iter()
                .filter(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + u64::from(*d - b'0'))
        })
        .parse_next(input)
}

fn parse_times(input: &mut &BStr) -> PResult<u64> {
    preceded((b"Time:", space0), parse_num).parse_next(input)
}

fn parse_distances(input: &mut &BStr) -> PResult<u64> {
    preceded((b"Distance:", space0), parse_num).parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Race> {
    let (time, dist) = separated_pair(parse_times, "\n", parse_distances).parse_next(input)?;
    Ok(Race { time, dist })
}

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read(std::env::var("INPUT")?)?;

    let Race { time, dist } = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut res = 1;
    let mut count = 0;
    for t in 1..time {
        let d = t * (time - t);
        if d > dist {
            count += 1;
        }
    }
    res *= count;

    println!("ANSWER: {res}");
    Ok(())
}
