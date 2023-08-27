// https://atcoder.jp/contests/abc317/tasks/abc317_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    let mut M = 0;
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
            Z: usize,
        }
        M += Z;
        if X > Y {
            vec.push((0, Z));
        }
        else {
            let xy = X + Y;
            vec.push(((xy + 1) / 2 - X, Z));
        }
    }
    let mut DP = vec![vec![INF; M+1]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        for j in 0..=M {
            DP[i+1][j] = min(DP[i+1][j], DP[i][j]);
            if j + vec[i].1 <= M {
                DP[i+1][j+vec[i].1] = min(DP[i+1][j+vec[i].1], DP[i][j] + vec[i].0);
            }
        }
    }
    println!("{}", DP[N][(M+1)/2..].iter().min().unwrap());
}