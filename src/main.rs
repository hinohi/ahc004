use std::time::{Duration, Instant};

use proconio::{input, marker::Bytes};
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Mcg128Xsl64;

const N: usize = 20;

fn empty_count(x: usize, s: &[u8], row: &[Option<u8>; N]) -> Option<usize> {
    let mut count = 0;
    for (s, i) in s.iter().zip(row[x..].iter()) {
        if i.is_none() {
            count += 1;
        } else if *i != Some(*s) {
            return None;
        }
    }
    if x + s.len() <= N {
        Some(count)
    } else {
        for (s, i) in s[N - x..].iter().zip(row.iter()) {
            if i.is_none() {
                count += 1;
            } else if *i != Some(*s) {
                return None;
            }
        }
        Some(count)
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
    for s in ss {
        let mut best_match = None;
        let mut best_pos = (0, 0, 0);
        for &y in yy.iter() {
            let row = dna.get(y).unwrap();
            for &x in xx.iter() {
                if let Some(m) = empty_count(x, s, row) {
                    if best_match.is_none() || best_match.unwrap() > m {
                        best_match = Some(m);
                        best_pos = (0, x, y);
                    }
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
                if let Some(m) = empty_count(y, s, &row) {
                    if best_match.is_none() || best_match.unwrap() > m {
                        best_match = Some(m);
                        best_pos = (1, x, y);
                    }
                }
            }
        }
        if best_match.is_some() {
            count += 1;
            if best_pos.0 == 0 {
                for (i, &c) in s.iter().enumerate() {
                    dna[best_pos.2][(best_pos.1 + i) % N] = Some(c);
                }
            } else {
                for (i, &c) in s.iter().enumerate() {
                    dna[(best_pos.2 + i) % N][best_pos.1] = Some(c);
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
            assert_eq!(empty_count(x, &s, &[None; N]), Some(12));
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
        assert_eq!(empty_count(0, &s, &row), Some(0));
        assert_eq!(empty_count(1, &s, &row), None);
        assert_eq!(empty_count(2, &s, &row), None);
        assert_eq!(empty_count(18, &s, &row), None);
        assert_eq!(empty_count(19, &s, &row), None);
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
        assert_eq!(empty_count(18, &s, &row), Some(0));
        assert_eq!(empty_count(17, &s, &row), None);
        assert_eq!(empty_count(19, &s, &row), None);
        assert_eq!(empty_count(0, &s, &row), None);
        assert_eq!(empty_count(1, &s, &row), None);
    }

    #[test]
    fn test_row_contain_wrap2() {
        let s = vec![b'H', b'A', b'F', b'C', b'H', b'D'];
        #[rustfmt::skip]
            let row = [
            None, Some(b'A'), Some(b'F'), None, None,
            None, None, None, None, None,
            None, None, None, None, None,
            None, None, None, None, None,
        ];
        assert_eq!(empty_count(0, &s, &row), Some(4));
        assert_eq!(empty_count(1, &s, &row), None);
        assert_eq!(empty_count(18, &s, &row), None);
        assert_eq!(empty_count(19, &s, &row), None);
    }
}
