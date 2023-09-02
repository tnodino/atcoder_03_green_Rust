// https://atcoder.jp/contests/abc297/tasks/abc297_e

use proconio::input;
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let mut ans = vec![0; K+1];
    let mut bh = BinaryHeap::new();
    for i in 0..N {
        bh.push(Reverse(A[i]));
    }
    for i in 1..=K {
        loop {
            let x = bh.pop().unwrap().0;
            if ans[i-1] == x {
                continue;
            }
            ans[i] = x;
            for j in 0..N {
                bh.push(Reverse(A[j] + x));
            }
            break;
        }
    }
    println!("{}", ans[K]);
}