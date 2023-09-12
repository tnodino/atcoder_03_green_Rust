// https://atcoder.jp/contests/abc219/tasks/abc219_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<30;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X, Y): (usize, usize, usize),
    }
    let mut DP = vec![vec![vec![INF; Y+1]; X+1]; N+1];
    DP[0][0][0] = 0;
    for i in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        for j in 0..=X {
            for k in 0..=Y {
                DP[i+1][j][k] = min(DP[i+1][j][k], DP[i][j][k]);
                let a = match j + A <= X {
                    true => j + A,
                    false => X,
                };
                let b = match k + B <= Y {
                    true => k + B,
                    false => Y,
                };
                DP[i+1][a][b] = min(DP[i+1][a][b], DP[i][j][k] + 1);
            }
        }
    }
    match DP[N][X][Y] == INF {
        true => println!("-1"),
        false => println!("{}", DP[N][X][Y]),
    }
}