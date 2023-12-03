use color_eyre::eyre::eyre;

#[aoc_macro::main("day3")]
fn main() -> color_eyre::Result<u32> {
    let re = regex::bytes::Regex::new(r"\d+")?;
    let mat: Vec<_> = input.split(|&c| c == b'\n').collect();
    let m = mat.len();
    let n = mat[0].len();

    let mut sum = 0;
    for (i, line) in mat.iter().enumerate() {
        for num in re.find_iter(line) {
            let start = num.start();
            let end = num.end();
            let num: u32 = atoi_simd::parse(num.as_bytes()).map_err(|e| eyre!("{e}"))?;

            if start > 0 && line[start - 1] != b'.' || end < n - 1 && line[end] != b'.' {
                println!("{}: left or right: {start}-{end}: {}", i + 1, num,);
                sum += num;
            } else {
                for j in start..end {
                    if i > 0 && mat[i - 1][j] != b'.' {
                        println!("{}: top: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                    if i < mat.len() - 1 && mat[i + 1][j] != b'.' {
                        println!("{}: bottom: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                    if i > 0 && j > 0 && mat[i - 1][j - 1] != b'.' {
                        println!("{}: top left: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                    if i < m - 1 && j > 0 && mat[i + 1][j - 1] != b'.' {
                        println!("{}: bottom left: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                    if i > 0 && j < n - 1 && mat[i - 1][j + 1] != b'.' {
                        println!("{}: top right: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                    if i < m - 1 && j < n - 1 && mat[i + 1][j + 1] != b'.' {
                        println!("{}: bottom right: {start}-{end}: {}", i + 1, num);
                        sum += num;
                        break;
                    }
                }
            }
        }
    }

    Ok(sum)
}
