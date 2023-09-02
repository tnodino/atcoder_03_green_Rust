// https://atcoder.jp/contests/abc305/tasks/abc305_e

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let mut bh = BinaryHeap::new();
    let mut cost = vec![-1; N];
    for _ in 0..K {
        input! {
            p: usize,
            h: isize,
        }
        bh.push((h, p-1));
    }
    while !bh.is_empty() {
        let (h, p) = bh.pop().unwrap();
        if h <= cost[p] {
            continue;
        }
        cost[p] = h;
        if h == 0 {
            continue;
        }
        for nxt in G[p].iter() {
            bh.push((h-1, *nxt));
        }
    }
    let mut vec = Vec::new();
    for i in 0..N {
        if cost[i] >= 0 {
            vec.push(i+1);
        }
    }
    println!("{}", vec.len());
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}