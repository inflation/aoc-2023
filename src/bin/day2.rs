use color_eyre::eyre::eyre;
use winnow::{
    ascii::{alpha1, digit1},
    combinator::{delimited, preceded, separated},
    PResult, Parser,
};

#[derive(Debug, Default)]
struct Game {
    red: Vec<u32>,
    blue: Vec<u32>,
    green: Vec<u32>,
}

fn parse_ball<'i>(input: &mut &'i [u8]) -> PResult<(&'i [u8], u32)> {
    let num = match atoi_simd::parse_pos(digit1.parse_next(input)?) {
        Ok(num) => num,
        Err(e) => {
            panic!("Failed to parse number: {e}")
        }
    };
    let ball = preceded(" ", alpha1).parse_next(input)?;
    Ok((ball, num))
}

fn parse_set(input: &mut &[u8]) -> PResult<(u32, u32, u32)> {
    let balls: Vec<(&[u8], u32)> = separated(1.., parse_ball, ", ").parse_next(input)?;
    let mut set = (0, 0, 0);
    for (ball, num) in balls {
        match ball {
            b"red" => set.0 = num,
            b"blue" => set.1 = num,
            b"green" => set.2 = num,
            _ => panic!("Unknown ball color: {}", String::from_utf8_lossy(ball)),
        }
    }
    Ok(set)
}

fn parse_line(input: &mut &[u8]) -> PResult<Game> {
    let sets: Vec<(u32, u32, u32)> = preceded(
        delimited(b"Game ", digit1, b": "),
        separated(1.., parse_set, "; "),
    )
    .parse_next(input)?;

    let mut game = Game::default();
    for (red, blue, green) in sets {
        game.red.push(red);
        game.blue.push(blue);
        game.green.push(green);
    }

    Ok(game)
}

fn parse_input(input: &mut &[u8]) -> PResult<Vec<Game>> {
    let res = separated(1.., parse_line, "\n").parse_next(input)?;
    Ok(res)
}

#[aoc_macro::main("day2")]
fn main() -> color_eyre::Result<u32> {
    let remain = &mut input.as_ref();
    let res = parse_input.parse(remain).map_err(|e| eyre!("{e}"))?;

    let mut sum = 0;
    for game in res {
        let power = game.red.iter().max().unwrap()
            * game.blue.iter().max().unwrap()
            * game.green.iter().max().unwrap();
        sum += power;
    }

    Ok(sum)
}
