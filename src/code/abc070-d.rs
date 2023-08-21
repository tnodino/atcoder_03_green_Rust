// https://atcoder.jp/contests/abc070/tasks/abc070_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        G[a-1].push((b-1, c));
        G[b-1].push((a-1, c));
    }
    input! {
        Q: usize,
        K: usize,
    }
    let mut dist = vec![INF; N];
    dist[K-1] = 0;
    let mut que = VecDeque::new();
    que.push_back(K-1);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (nxt, c) in G[pos].iter() {
            if dist[*nxt] == INF {
                dist[*nxt] = dist[pos] + c;
                que.push_back(*nxt);
            }
        }
    }
    for _ in 0..Q {
        input! {
            x: usize,
            y: usize,
        }
        println!("{}", dist[x-1] + dist[y-1]);
    }
}