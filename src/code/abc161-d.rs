// https://atcoder.jp/contests/abc161/tasks/abc161_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 1..=9 {
        que.push_back(i);
    }
    for _ in 0..K-1 {
        let x = que.pop_front().unwrap();
        if x % 10 != 0 {
            que.push_back(x * 10 + x % 10 - 1);
        }
        que.push_back(x * 10 + x % 10);
        if x % 10 != 9 {
            que.push_back(x * 10 + x % 10 + 1);
        }
    }
    println!("{}", que.pop_front().unwrap());
}