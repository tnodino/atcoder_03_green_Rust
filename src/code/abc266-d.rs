// https://atcoder.jp/contests/abc266/tasks/abc266_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const INF: isize = -(1<<50);

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 100_000;
    let mut s = vec![vec![0; 5]; M+1];
    for _ in 0..N {
        input! {
            T: usize,
            X: usize,
            A: isize,
        }
        s[T][X] += A;
    }
    let mut DP = vec![vec![INF; 5]; M+1];
    DP[0][0] = 0;
    for i in 0..M {
        for j in 0..5 {
            if j > 0 {
                DP[i+1][j-1] = max(DP[i+1][j-1], DP[i][j] + s[i+1][j-1]);
            }
            DP[i+1][j] = max(DP[i+1][j], DP[i][j] + s[i+1][j]);
            if j + 1 < 5 {
                DP[i+1][j+1] = max(DP[i+1][j+1], DP[i][j] + s[i+1][j+1]);
            }
        }
    }
    println!("{}", DP[M].iter().max().unwrap());
}