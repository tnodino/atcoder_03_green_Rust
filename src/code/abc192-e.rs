// https://atcoder.jp/contests/abc192/tasks/abc192_e

use proconio::input;
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: usize,
        Y: usize,
    }
    let mut G = vec![Vec::new(); N+1];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
            T: usize,
            K: usize,
        }
        G[A].push((B, T, K));
        G[B].push((A, T, K));
    }
    let mut cost = vec![INF; N+1];
    cost[X] = 0;
    let mut bh = BinaryHeap::new();
    bh.push((Reverse(0), X));
    while !bh.is_empty() {
        let (now, pos) = bh.pop().unwrap();
        let now = now.0;
        if cost[pos] < now {
            continue;
        }
        for (nxt, t, k) in G[pos].iter() {
            let arr = ((now + k - 1) / k) * k + t;
            if arr < cost[*nxt] {
                cost[*nxt] = arr;
                bh.push((Reverse(arr), *nxt));
            }
        }
    }
    if cost[Y] == INF {
        println!("-1");
    }
    else {
        println!("{}", cost[Y]);
    }
}