// https://atcoder.jp/contests/abc188/tasks/abc188_e

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [isize; N],
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            (X, Y): (usize, usize),
        }
        G[X-1].push(Y-1);
    }
    let mut cost = vec![INF; N];
    let mut ans = -INF;
    for i in 0..N {
        if cost[i] < INF {
            ans = max(ans, A[i] - cost[i]);
        }
        cost[i] = min(cost[i], A[i]);
        for nxt in G[i].iter() {
            cost[*nxt] = min(cost[*nxt], cost[i]);
        }
    }
    println!("{}", ans);
}