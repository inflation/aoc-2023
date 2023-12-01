use color_eyre::eyre::eyre;
use regex::bytes::Regex;

#[aoc_macro::main("day1")]
fn main() -> color_eyre::Result<u32> {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")?;

    let mut sum = 0;
    for line in input.split(|&c| c == b'\n') {
        let mut first = None;
        let mut last = None;

        for m in re.find_iter(line) {
            if let Some(d) = to_digit(m.as_bytes()) {
                if first.is_none() {
                    first = Some(d);
                }

                last = Some(d);
            }
        }

        if let (Some(first), Some(last)) = (first, last) {
            sum += u32::from(first * 10 + last);
        } else {
            return Err(eyre!("Could not find digits in line: {line:?}"));
        }
    }

    Ok(sum)
}

fn to_digit(word: &[u8]) -> Option<u8> {
    Some(match word {
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        m if !m.is_empty() => m[0] - b'0',
        _ => return None,
    })
}
