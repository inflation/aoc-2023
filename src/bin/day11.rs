use std::ops::{Index, IndexMut};

use color_eyre::eyre::eyre;
use itertools::Itertools;
use winnow::{
    combinator::{repeat, separated},
    token::one_of,
    BStr, PResult, Parser,
};

#[derive(Debug)]
struct Image {
    img: Vec<u8>,
    width: u32,
    height: u32,
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "w: {}, h: {}", self.width, self.height)?;
        for row in self.img.chunks(self.width as usize) {
            for &c in row {
                write!(f, "{}", c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(u32, u32)> for Image {
    type Output = u8;
    fn index(&self, (i, j): (u32, u32)) -> &Self::Output {
        &self.img[(i * self.width + j) as usize]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (i, j): (u32, u32)) -> &mut Self::Output {
        &mut self.img[(i * self.width + j) as usize]
    }
}

fn parse_line(input: &mut &BStr) -> PResult<()> {
    repeat(1.., one_of(b".#")).parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Image> {
    separated(1.., parse_line.recognize(), "\n")
        .map(|v: Vec<&[u8]>| {
            let width = v[0].len();
            let height = v.len();
            let img = v.into_iter().flatten().copied().collect();
            Image {
                img,
                width: u32::try_from(width).unwrap(),
                height: u32::try_from(height).unwrap(),
            }
        })
        .parse_next(input)
}

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read(std::env::var("INPUT")?)?;

    let image = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;
    let empty_rows: Vec<usize> = image
        .img
        .chunks_exact(image.width as usize)
        .enumerate()
        .filter_map(|(i, x)| if x.contains(&b'#') { None } else { Some(i) })
        .collect();
    let t = transpose(&image);
    let empty_cols: Vec<usize> = t
        .chunks_exact(image.height as usize)
        .enumerate()
        .filter_map(|(i, x)| if x.contains(&b'#') { None } else { Some(i) })
        .collect();

    let galaxies: Vec<(u32, u32)> = image
        .img
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == b'#')
        .map(|(i, _)| {
            let i = u32::try_from(i).unwrap();
            (i / image.width, i % image.width)
        })
        .collect();

    let mut sum = 0;
    for (g1, g2) in galaxies.into_iter().tuple_combinations() {
        let d = dist(g1, g2, &empty_rows, &empty_cols, 999999);
        sum += d;
    }

    println!("ANSWER: {sum}");
    Ok(())
}

fn dist((x1, y1): (u32, u32), (x2, y2): (u32, u32), e_r: &[usize], e_c: &[usize], mul: u64) -> u64 {
    let (x1, x2) = (x1.min(x2), x1.max(x2));
    let (y1, y2) = (y1.min(y2), y1.max(y2));

    let mut res = 0;
    res += (x2 - x1) as u64
        + mul
            * e_r
                .iter()
                .filter(|&&x| (x1..x2).contains(&(x as u32)))
                .count() as u64;
    res += (y2 - y1) as u64
        + mul
            * e_c
                .iter()
                .filter(|&&y| (y1..y2).contains(&(y as u32)))
                .count() as u64;
    res
}

fn transpose(image: &Image) -> Vec<u8> {
    let mut new_img = vec![0; image.height as usize * image.width as usize];
    for i in 0..image.height {
        for j in 0..image.width {
            new_img[(j * image.height + i) as usize] = image[(i, j)];
        }
    }

    new_img
}
