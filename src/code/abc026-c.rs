// https://atcoder.jp/contests/abc026/tasks/abc026_c

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for i in 1..N {
        input! {
            B: usize,
        }
        G[B-1].push(i);
    }
    let mut DP = vec![1; N];
    for i in (0..N).rev() {
        if G[i].is_empty() {
            continue;
        }
        let mut ma = 0;
        let mut mi = INF;
        for j in G[i].iter() {
            ma = max(ma, DP[*j]);
            mi = min(mi, DP[*j]);
        }
        DP[i] += ma + mi;
    }
    println!("{}", DP[0]);
}