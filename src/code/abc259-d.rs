// https://atcoder.jp/contests/abc259/tasks/abc259_d

use proconio::input;
use proconio::fastout;
use num::pow;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, sx, sy, tx, ty): (usize, isize, isize, isize, isize),
    }
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut r = Vec::new();
    for _ in 0..N {
        input! {
            (i1, i2, i3): (isize, isize, isize),
        }
        x.push(i1);
        y.push(i2);
        r.push(i3);
    }
    let mut G = vec![Vec::new(); N+2];
    for i in 0..N {
        let dist = pow(x[i] - sx, 2) + pow(y[i] - sy, 2);
        if dist == pow(r[i], 2) {
            G[N].push(i);
        }
        let dist = pow(x[i] - tx, 2) + pow(y[i] - ty, 2);
        if dist == pow(r[i], 2) {
            G[i].push(N+1);
        }
        for j in i+1..N {
            let dist = pow(x[i] - x[j], 2) + pow(y[i] - y[j], 2);
            if pow(r[i] - r[j], 2) <= dist && dist <= pow(r[i] + r[j], 2) {
                G[i].push(j);
                G[j].push(i);
            }
        }
    }
    let mut flg = vec![false; N+2];
    flg[N] = true;
    let mut que = VecDeque::from([N]);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if !flg[*nxt] {
                flg[*nxt] = true;
                que.push_back(*nxt);
            }
        }
    }
    println!("{}", match flg[N+1] {
        true => "Yes",
        false => "No",
    })
}