use std::time::{Duration, Instant};

use proconio::{input, marker::Bytes};
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Mcg128Xsl64;

const N: usize = 20;

fn row_contain(x: usize, s: &[u8], row: &[Option<u8>; N]) -> bool {
    let a = s
        .iter()
        .zip(row[x..].iter())
        .all(|(s, i)| i.is_none() || *i == Some(*s));
    if x + s.len() <= N {
        a
    } else if a {
        s[N - x..]
            .iter()
            .zip(row.iter())
            .all(|(s, i)| i.is_none() || *i == Some(*s))
    } else {
        false
    }
}

fn match_dna<R: Rng>(rng: &mut R, ss: &[Vec<u8>]) -> (u64, [[Option<u8>; N]; N]) {
    let xx = {
        let mut xx = (0..N).collect::<Vec<_>>();
        xx.shuffle(rng);
        xx
    };
    let yy = {
        let mut xx = (0..N).collect::<Vec<_>>();
        xx.shuffle(rng);
        xx
    };
    let mut count = 0;
    let mut dna = [[None; N]; N];
    'OUT: for s in ss {
        for &y in yy.iter() {
            let row = dna.get_mut(y).unwrap();
            for &x in xx.iter() {
                if row_contain(x, s, row) {
                    for (i, c) in s.iter().enumerate() {
                        row[(x + i) % N] = Some(*c);
                    }
                    count += 1;
                    continue 'OUT;
                }
            }
        }
        for &x in yy.iter() {
            let row = {
                let mut row = [None; N];
                for y in 0..N {
                    row[y] = dna[y][x];
                }
                row
            };
            for &y in xx.iter() {
                if row_contain(y, s, &row) {
                    for (i, c) in s.iter().enumerate() {
                        dna[(y + i) % N][x] = Some(*c);
                    }
                    count += 1;
                    continue 'OUT;
                }
            }
        }
    }
    let score = if count == ss.len() {
        let mut d = 0;
        for row in dna.iter() {
            for c in row.iter() {
                if c.is_none() {
                    d += 1;
                }
            }
        }
        let n2 = 2.0 * (N as f64) * (N as f64);
        (1e8 * n2 / (n2 - d as f64)).round() as u64
    } else {
        (1e8 * count as f64 / ss.len() as f64).round() as u64
    };
    (score, dna)
}

fn print_dna(dna: &[[Option<u8>; N]; N]) {
    for row in dna.iter() {
        for c in row.iter() {
            if let Some(c) = c {
                print!("{}", std::char::from_u32(*c as u32).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let start = Instant::now();
    input! {
        n: usize,
        m: usize,
        mut ss: [Bytes; m],
    }
    assert_eq!(n, N);

    let mut rng = Mcg128Xsl64::new(1);
    let mut best_score = 0;
    let mut best_dna = [[None; N]; N];
    while start.elapsed() < Duration::from_millis(2800) {
        let (score, dna) = match_dna(&mut rng, &ss);
        if score > best_score {
            best_score = score;
            best_dna = dna;
            eprintln!("{}", best_score);
        }
        ss.shuffle(&mut rng);
    }
    print_dna(&best_dna);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_contain_all_none() {
        let s = vec![b'A'; 12];
        for x in 0..N {
            assert!(row_contain(x, &s, &[None; N]));
        }
    }

    #[test]
    fn test_row_contain_trivial() {
        let s = vec![b'A', b'B', b'C'];
        #[rustfmt::skip]
        let row = [
            Some(b'A'), Some(b'B'), Some(b'C'), None, None,
            None, None, None, None, None,
            None, None, None, None, None,
            None, None, None, None, None,
        ];
        assert!(row_contain(0, &s, &row));
        assert!(!row_contain(1, &s, &row));
        assert!(!row_contain(2, &s, &row));
        assert!(!row_contain(18, &s, &row));
        assert!(!row_contain(19, &s, &row));
    }

    #[test]
    fn test_row_contain_wrap() {
        let s = vec![b'H', b'A', b'B', b'C'];
        #[rustfmt::skip]
            let row = [
            Some(b'B'), Some(b'C'), None, None, None,
            None, None, None, None, None,
            None, None, None, None, None,
            None, None, None, Some(b'H'), Some(b'A'),
        ];
        assert!(row_contain(18, &s, &row));
        assert!(!row_contain(17, &s, &row));
        assert!(!row_contain(19, &s, &row));
        assert!(!row_contain(0, &s, &row));
        assert!(!row_contain(1, &s, &row));
    }

    #[test]
    fn test_row_contain_wrap2() {
        let s = vec![b'H', b'D', b'B', b'C', b'H', b'D'];
        #[rustfmt::skip]
            let row = [
            None, Some(b'A'), Some(b'F'), None, None,
            None, None, None, None, None,
            None, None, None, None, None,
            None, None, None, None, None,
        ];
        assert!(!row_contain(0, &s, &row));
        assert!(!row_contain(1, &s, &row));
        assert!(!row_contain(18, &s, &row));
        assert!(!row_contain(19, &s, &row));
    }
}
