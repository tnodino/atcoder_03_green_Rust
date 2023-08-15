// https://atcoder.jp/contests/abc075/tasks/abc075_c

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
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        vec.push((a-1, b-1));
    }
    let mut ans = 0;
    for i in 0..M {
        let mut G = vec![Vec::new(); N];
        for j in 0..M {
            if i == j {
                continue;
            }
            G[vec[j].0].push(vec[j].1);
            G[vec[j].1].push(vec[j].0);
        }
        let mut flg = vec![false; N];
        flg[0] = true;
        let mut que = VecDeque::new();
        que.push_back(0);
        while !que.is_empty() {
            let pos = que.pop_front().unwrap();
            for nxt in G[pos].iter() {
                if flg[*nxt] {
                    continue;
                }
                flg[*nxt] = true;
                que.push_back(*nxt);
            }
        }
        if !flg.iter().all(|&x| x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}