// https://atcoder.jp/contests/abc012/tasks/abc012_4

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut cost = vec![vec![INF; N]; N];
    for i in 0..N {
        cost[i][i] = 0;
    }
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
            t: usize,
        }
        cost[a-1][b-1] = t;
        cost[b-1][a-1] = t;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
            }
        }
    }
    let mut ans = INF;
    for i in 0..N {
        ans = min(ans, *cost[i].iter().max().unwrap());
    }
    println!("{}", ans);
}