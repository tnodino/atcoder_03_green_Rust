// https://atcoder.jp/contests/abc216/tasks/abc216_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut cnt = vec![0; N];
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            k: usize,
            a: [usize; k],
        }
        cnt[a[0]-1] += 1;
        for i in 0..k-1 {
            G[a[i]-1].push(a[i+1]-1);
        }
    }
    let mut que = VecDeque::new();
    for i in 0..N {
        if cnt[i] == 2 {
            que.push_back(i);
        }
    }
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            cnt[*nxt] += 1;
            if cnt[*nxt] == 2 {
                que.push_back(*nxt);
            }
        }
    }
    println!("{}", match cnt.iter().all(|&x| x == 2) {
        true => "Yes",
        false => "No",
    })
}