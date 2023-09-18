// https://atcoder.jp/contests/abc223/tasks/abc223_d

use proconio::input;
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut G = vec![Vec::new(); N];
    let mut cnt = vec![0; N];
    for _ in 0..M {
        input! {
            (A, B): (usize, usize),
        }
        G[A-1].push(B-1);
        cnt[B-1] += 1;
    }
    let mut bh = BinaryHeap::new();
    for i in 0..N {
        if cnt[i] == 0 {
            bh.push(Reverse(i));
        }
    }
    let mut top = Vec::new();
    while !bh.is_empty() {
        let pos = bh.pop().unwrap().0;
        top.push(pos + 1);
        for nxt in G[pos].iter() {
            cnt[*nxt] -= 1;
            if cnt[*nxt] == 0 {
                bh.push(Reverse(*nxt));
            }
        }
    }
    match top.len() == N {
        true => println!("{}", top.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ")),
        false => println!("-1"),
    }
}