// https://atcoder.jp/contests/abc016/tasks/abc016_3

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N+1];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        G[A].push(B);
        G[B].push(A);
    }
    for i in 1..=N {
        let mut dist = vec![-1; N+1];
        dist[i] = 0;
        let mut que = VecDeque::new();
        que.push_back(i);
        while !que.is_empty() {
            let pos = que.pop_front().unwrap();
            for nxt in G[pos].iter() {
                if dist[*nxt] == -1 {
                    dist[*nxt] = dist[pos] + 1;
                    que.push_back(*nxt);
                }
            }
        }
        println!("{}", dist.iter().filter(|&x| *x == 2).count());
    }
}