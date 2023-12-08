use aoc_2023::atoi;
use color_eyre::eyre::eyre;
use log::info;
use regex::bytes::Regex;

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read(std::env::var("INPUT")?)?;

    let re_star = regex::bytes::Regex::new(r"\*")?;
    let re_num = regex::bytes::Regex::new(r"\d+")?;

    let mat: Vec<_> = input.split(|&c| c == b'\n').collect();
    let m = mat.len();
    let n = mat[0].len();

    let parts: Vec<Vec<_>> = mat
        .iter()
        .map(|line| re_num.find_iter(line).collect())
        .collect();

    let sum = find_gear(&mat, &re_star, &parts, n, m)?;

    println!("ANSWER: {sum}");
    Ok(())
}

fn find_gear(
    mat: &[&[u8]],
    re_star: &Regex,
    parts: &[Vec<regex::bytes::Match<'_>>],
    n: usize,
    m: usize,
) -> Result<u32, color_eyre::eyre::Error> {
    let mut sum = 0;
    for (i, line) in mat.iter().enumerate() {
        for gear in re_star.find_iter(line) {
            let gear = gear.start();
            let mut count = 0;
            let mut ratio = 1;

            if gear > 0 && line[gear - 1].is_ascii_digit() {
                info!("{}: left: {gear}", i + 1,);
                count += 1;
                let part = atoi(
                    parts[i]
                        .iter()
                        .find(|&m| m.end() == gear)
                        .ok_or_else(|| eyre!("Parts not found: {} at line {}", gear - 1, i + 1))?
                        .as_bytes(),
                );
                ratio *= part;
            }
            if gear < n - 1 && line[gear + 1].is_ascii_digit() {
                info!("{}: right: {gear}", i + 1,);
                count += 1;
                let part = atoi(
                    parts[i]
                        .iter()
                        .find(|&m| m.start() == gear + 1)
                        .ok_or_else(|| eyre!("Parts not found: {} at line {}", gear + 1, i + 1))?
                        .as_bytes(),
                );
                ratio *= part;
            }
            if i > 0 && mat[i - 1][gear].is_ascii_digit() {
                if let Some(part) = parts[i - 1].iter().find(|&m| m.range().contains(&gear)) {
                    info!("{}: up: {gear}", i + 1);
                    count += 1;
                    let part = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: up");
                }
            }
            if i < n - 1 && mat[i + 1][gear].is_ascii_digit() {
                if let Some(part) = parts[i + 1].iter().find(|&m| m.range().contains(&gear)) {
                    info!("{}: down: {gear}", i + 1,);
                    count += 1;
                    let part = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: down");
                }
            }
            if gear > 0 && i > 0 && mat[i - 1][gear - 1].is_ascii_digit() {
                if let Some(part) = parts[i - 1].iter().find(|&m| m.end() == gear) {
                    info!("{}: up-left: {gear}", i + 1,);
                    count += 1;
                    let part = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: up-left");
                }
            }
            if gear < n - 1 && i > 0 && mat[i - 1][gear + 1].is_ascii_digit() {
                if let Some(part) = parts[i - 1].iter().find(|&m| m.start() == gear + 1) {
                    info!("{}: up-right: {gear}", i + 1,);
                    count += 1;
                    let part = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: up-right");
                }
            }
            if gear > 0 && i < m - 1 && mat[i + 1][gear - 1].is_ascii_digit() {
                if let Some(part) = parts[i + 1].iter().find(|&m| m.end() == gear) {
                    info!("{}: down-left: {gear}", i + 1,);
                    count += 1;
                    let part: u32 = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: down-left");
                }
            }
            if gear < n - 1 && i < m - 1 && mat[i + 1][gear + 1].is_ascii_digit() {
                if let Some(part) = parts[i + 1].iter().find(|&m| m.start() == gear + 1) {
                    info!("{}: down-right: {gear}", i + 1,);
                    count += 1;
                    let part: u32 = atoi(part.as_bytes());
                    ratio *= part;
                } else {
                    info!("Part not found: down-right");
                }
            }

            if count == 2 {
                sum += ratio;
            }
        }
    }
    Ok(sum)
}
