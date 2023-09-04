// https://atcoder.jp/contests/abc267/tasks/abc267_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const INF: isize = -1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; N],
    }
    let mut DP = vec![vec![INF; M+1]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        for j in 0..=M {
            DP[i+1][j] = max(DP[i+1][j], DP[i][j]);
            if j + 1 <= M {
                DP[i+1][j+1] = max(DP[i+1][j+1], DP[i][j] + A[i] * (j as isize + 1));
            }
        }
    }
    println!("{}", DP[N][M]);
}