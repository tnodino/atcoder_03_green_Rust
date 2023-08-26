// https://atcoder.jp/contests/abc261/tasks/abc261_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const INF: isize = -(1<<50);

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: [isize; N],
    }
    let mut bonus = vec![0; N];
    for _ in 0..M {
        input! {
            C: usize,
            Y: isize,
        }
        bonus[C-1] = Y;
    }
    let mut DP = vec![vec![INF; N+1]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        for j in 0..N {
            DP[i+1][0] = max(DP[i+1][0], DP[i][j]);
            DP[i+1][j+1] = max(DP[i+1][j+1], DP[i][j] + X[i] + bonus[j]);
        }
    }
    println!("{}", DP[N].iter().max().unwrap());
}