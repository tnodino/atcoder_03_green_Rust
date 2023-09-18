// https://atcoder.jp/contests/abc281/tasks/abc281_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const INF: isize = -1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K, D): (usize, usize, usize),
        a: [isize; N],
    }
    let mut DP = vec![vec![vec![INF; D]; K+1]; N+1];
    DP[0][0][0] = 0;
    for i in 0..N {
        for j in 0..=K {
            for k in 0..D {
                DP[i+1][j][k] = max(DP[i+1][j][k], DP[i][j][k]);
                if j < K {
                    let idx = (k + a[i] as usize) % D;
                    DP[i+1][j+1][idx] = max(DP[i+1][j+1][idx], DP[i][j][k] + a[i]);
                }
            }
        }
    }
    match DP[N][K][0] < 0 {
        true => println!("-1"),
        false => println!("{}", DP[N][K][0]),
    }
}