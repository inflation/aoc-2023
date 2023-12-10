use std::collections::VecDeque;

use color_eyre::eyre::eyre;
use winnow::{
    ascii::digit1,
    combinator::{opt, separated},
    BStr, PResult, Parser,
};

fn parse_num(input: &mut &BStr) -> PResult<i32> {
    (opt("-"), digit1).recognize().parse_to().parse_next(input)
}

fn parse_ns(input: &mut &BStr) -> PResult<Vec<i32>> {
    separated(1.., parse_num, " ").parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Vec<Vec<i32>>> {
    separated(1.., parse_ns, "\n").parse_next(input)
}

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read(std::env::var("INPUT")?)?;

    let report = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let mut sum = 0;
    for hist in report {
        let mut reduce = hist;
        let mut traces = vec![VecDeque::from(reduce.clone())];
        loop {
            reduce = reduce.windows(2).map(|w| w[1] - w[0]).collect();
            traces.push(VecDeque::from(reduce.clone()));
            if !reduce.iter().any(|&x| x != 0) {
                break;
            }
        }

        traces.last_mut().unwrap().push_front(0);
        for i in (1..traces.len()).rev() {
            let first = traces[i - 1].front().unwrap() - traces[i].front().unwrap();
            traces[i - 1].push_front(first);
        }
        sum += traces[0].front().unwrap();
    }

    println!("ANSWER: {sum}");
    Ok(())
}
