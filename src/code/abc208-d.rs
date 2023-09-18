// https://atcoder.jp/contests/abc208/tasks/abc208_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut dist = vec![vec![INF; N]; N];
    for i in 0..N {
        dist[i][i] = 0;
    }
    for _ in 0..M {
        input! {
            (A, B, C): (usize, usize, usize),
        }
        dist[A-1][B-1] = C;
    }
    let mut ans = 0;
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
                if dist[i][j] < INF {
                    ans += dist[i][j];
                }
            }
        }
    }
    println!("{}", ans);
}