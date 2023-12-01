use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = BufReader::new(File::open("input/day1.txt")?);

    let mut sum = 0;
    for line in input.lines() {
        let line = line?;

        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }

        if let Some(f) = first.and_then(|c| c.to_digit(10)) {
            if let Some(l) = last.and_then(|c| c.to_digit(10)) {
                sum += f * 10 + l;
            }
        }
    }

    println!("ANSWER: {sum}");

    Ok(())
}
