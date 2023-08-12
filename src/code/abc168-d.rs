// https://atcoder.jp/contests/abc168/tasks/abc168_d

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
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut ans = vec![0; N+1];
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if ans[*nxt] == 0 {
                ans[*nxt] = pos;
                que.push_back(*nxt);
            }
        }
    }
    println!("Yes");
    for i in 2..=N {
        println!("{}", ans[i]);
    }
}