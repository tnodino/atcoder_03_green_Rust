// https://atcoder.jp/contests/abc141/tasks/abc141_d

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }
    let mut bh = BinaryHeap::new();
    for a in A.iter() {
        bh.push(*a);
    }
    for _ in 0..M {
        let x = bh.pop().unwrap();
        bh.push(x/2);
    }
    let mut ans = 0;
    for _ in 0..N {
        ans += bh.pop().unwrap();
    }
    println!("{}", ans);
}