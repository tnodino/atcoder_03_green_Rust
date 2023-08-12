// https://atcoder.jp/contests/abc138/tasks/abc138_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let mut cost = vec![0; N];
    for _ in 0..Q {
        input! {
            p: usize,
            x: usize,
        }
        cost[p-1] += x;
    }
    let mut que = VecDeque::new();
    que.push_back((0, N));
    while !que.is_empty() {
        let (pos, rev) = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if *nxt == rev {
                continue;
            }
            cost[*nxt] += cost[pos];
            que.push_back((*nxt, pos));
        }
    }
    println!("{}", cost.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}