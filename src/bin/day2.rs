use winnow::{
    ascii::{alpha1, digit1},
    combinator::{delimited, preceded, repeat, separated, terminated},
    token::take_until1,
    PResult, Parser,
};

#[derive(Debug, Default)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_ball<'i>(input: &mut &'i [u8]) -> PResult<(&'i [u8], u32)> {
    let num = digit1.parse_next(input)?;
    let ball = preceded(" ", alpha1).parse_next(input)?;
    Ok((ball, atoi(num)))
}

fn parse_set(input: &mut &[u8]) -> PResult<Set> {
    let balls: Vec<(&[u8], u32)> = separated(1.., parse_ball, ", ").parse_next(input)?;
    let mut set = Set::default();
    for (ball, num) in balls {
        match ball {
            b"red" => set.red = num,
            b"blue" => set.blue = num,
            b"green" => set.green = num,
            _ => panic!("Unknown ball color: {}", String::from_utf8_lossy(ball)),
        }
    }
    Ok(set)
}

fn parse_id(input: &mut &[u8]) -> PResult<u32> {
    let id = take_until1(":").parse_next(input)?;
    Ok(atoi(id))
}

fn parse_line(input: &mut &[u8]) -> PResult<Vec<Set>> {
    let (_id, game) = terminated(
        (
            delimited(b"Game ", parse_id, b": "),
            separated(1.., parse_set, "; "),
        ),
        "\n",
    )
    .parse_next(input)?;
    Ok(game)
}

fn parse_input(input: &mut &[u8]) -> PResult<Vec<Vec<Set>>> {
    let res = repeat(0.., parse_line).parse_next(input)?;
    Ok(res)
}

#[aoc_macro::main("day2")]
fn main() -> color_eyre::Result<u32> {
    let remain = &mut input.as_ref();
    let res = parse_input(remain).unwrap();

    let mut sum = 0;
    for (i, sets) in res.iter().enumerate() {
        let i = i + 1;
        if sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        {
            // println!("Game {i} is possible: {sets:?}");
            sum += i;
        } else {
            println!("Game {i} is impossible: {sets:?}");
        }
    }

    Ok(sum)
}

fn atoi(input: &[u8]) -> u32 {
    input
        .iter()
        .fold(0, |acc, x| acc * 10 + u32::from(x - b'0'))
}
