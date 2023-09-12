// https://atcoder.jp/contests/agc043/tasks/agc043_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
    }
    let mut s = Vec::new();
    for _ in 0..H {
        input! {
            i1: String,
        }
        let i1 = i1.chars().collect::<Vec<char>>();
        s.push(i1);
    }
    let mut DP = vec![vec![INF; W]; H];
    DP[0][0] = match s[0][0] {
        '.' => 0,
        '#' => 1,
        _ => unreachable!(),
    };
    for i in 0..H {
        for j in 0..W {
            if i + 1 < H {
                let cost = DP[i][j] + match (s[i][j], s[i+1][j]) {
                    ('.', '#') => 1,
                    (_, _) => 0,
                };
                DP[i+1][j] = min(DP[i+1][j], cost);
            }
            if j + 1 < W {
                let cost = DP[i][j] + match (s[i][j], s[i][j+1]) {
                    ('.', '#') => 1,
                    (_, _) => 0,
                };
                DP[i][j+1] = min(DP[i][j+1], cost);
            }
        }
    }
    println!("{}", DP[H-1][W-1]);
}