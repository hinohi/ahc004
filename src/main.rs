use proconio::{input, marker::Bytes};

const N: usize = 20;

fn row_contain(x: usize, s: &[u8], row: &[Option<u8>; 20]) -> bool {
    let a = s
        .iter()
        .zip(row[x..].iter())
        .all(|(s, i)| i.is_none() || *i == Some(*s));
    if x + s.len() < row.len() {
        a
    } else if a {
        s[..N - x]
            .iter()
            .zip(row.iter())
            .all(|(s, i)| i.is_none() || *i == Some(*s))
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Bytes; m],
    }
    assert_eq!(n, N);

    let mut dna = [[None; N]; N];
    'OUT: for s in ss {
        for row in dna.iter_mut() {
            for x in 0..N {
                if row_contain(x, &s, row) {
                    for (i, c) in s.iter().enumerate() {
                        row[(x + i) % N] = Some(*c);
                    }
                    continue 'OUT;
                }
            }
        }
        for x in 0..N {
            let row = {
                let mut row = [None; N];
                for y in 0..N {
                    row[y] = dna[y][x];
                }
                row
            };
            for y in 0..N {
                if row_contain(y, &s, &row) {
                    for (i, c) in s.iter().enumerate() {
                        dna[(x + i) % N][x] = Some(*c);
                    }
                    continue 'OUT;
                }
            }
        }
    }

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
