use std::ops::{Index, IndexMut};

use color_eyre::eyre::eyre;
use winnow::{
    combinator::{repeat, separated},
    token::one_of,
    BStr, PResult, Parser,
};

#[derive(Debug)]
struct Map {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn position(&self, x: u8) -> (usize, usize) {
        let pos = self
            .map
            .iter()
            .position(|&y| y == x)
            .unwrap_or_else(|| panic!("pos not found: {x}"));
        (pos / self.width, pos % self.width)
    }
}

impl Index<(usize, usize)> for Map {
    type Output = u8;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.map[i * self.width + j]
    }
}

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.map[i * self.width + j]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Start,
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(input: &mut &BStr) -> PResult<()> {
    repeat(1.., one_of(b"|-LJ7F.S")).parse_next(input)
}

fn parse_input(input: &mut &BStr) -> PResult<Map> {
    separated(1.., parse_line.recognize(), "\n")
        .map(|v: Vec<&[u8]>| {
            let width = v[0].len();
            let height = v.len();
            let map = v.into_iter().flatten().copied().collect();
            Map { map, width, height }
        })
        .parse_next(input)
}

fn main() -> color_eyre::Result<()> {
    use Direction as D;

    let input = std::fs::read(std::env::var("INPUT")?)?;

    let mut map @ Map { width, height, .. } = parse_input.parse((*input).into()).map_err(|e| {
        eyre!(
            "cause: {:?}, remain: {:?}, offset: {}",
            e.inner(),
            String::from_utf8_lossy(&e.input()[e.offset()..]),
            e.offset(),
        )
    })?;

    let start = map.position(b'S');

    let mut loop_ = vec![start];
    let mut cursor = start;
    let mut last = D::Start;

    let (i, j) = cursor;
    if i > 0 && b"S7|F".contains(&map[(i - 1, j)]) {
        cursor = (i - 1, j);
        last = D::Up;
    } else if i < height - 1 && b"SJ|L".contains(&map[(i + 1, j)]) {
        cursor = (i + 1, j);
        last = D::Down;
    } else if j > 0 && b"SL-F".contains(&map[(i, j - 1)]) {
        cursor = (i, j - 1);
        last = D::Left;
    } else if j < width - 1 && b"SJ-7".contains(&map[(i, j + 1)]) {
        cursor = (i, j + 1);
        last = D::Right;
    }

    loop {
        let (i, j) = cursor;
        loop_.push(cursor);

        match (map[cursor], last) {
            (b'|', D::Up) | (b'L', D::Left) | (b'J', D::Right) => {
                cursor = (i - 1, j);
                last = D::Up;
            }
            (b'|', D::Down) | (b'7', D::Right) | (b'F', D::Left) => {
                cursor = (i + 1, j);
                last = D::Down;
            }
            (b'-', D::Left) | (b'J', D::Down) | (b'7', D::Up) => {
                cursor = (i, j - 1);
                last = D::Left;
            }
            (b'-', D::Right) | (b'L', D::Down) | (b'F', D::Up) => {
                cursor = (i, j + 1);
                last = D::Right;
            }
            _ => return Err(eyre!("invalid direction: {:?}", last)),
        }

        if cursor == start {
            break;
        }
    }

    map.map.iter_mut().enumerate().for_each(|(idx, f)| {
        let (i, j) = (idx / width, idx % width);
        if !loop_.contains(&(i, j)) {
            *f = b'.';
        }
    });

    let b = loop_.len();
    let twice_area = loop_.windows(2).fold(0_i32, |acc, xy| {
        let &[(x1, y1), (x2, y2)] = xy else {
            panic!("invalid windows");
        };
        acc + i32::try_from(y1 + y2).unwrap()
            * (i32::try_from(x1).unwrap() - i32::try_from(x2).unwrap())
    });
    let twice_i = twice_area.abs() - i32::try_from(b).unwrap() + 2;
    println!("ANSWER: {}", twice_i / 2);
    Ok(())
}
