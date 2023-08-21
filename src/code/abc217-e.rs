// https://atcoder.jp/contests/abc217/tasks/abc217_e

use proconio::input;
use proconio::fastout;
use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut bh: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: usize,
            }
            que.push_back(x);
        }
        else if q == 2 {
            if bh.is_empty() {
                println!("{}", que.pop_front().unwrap());
            }
            else {
                println!("{}", bh.pop().unwrap().0);
            }
        }
        else {
            while !que.is_empty() {
                bh.push(Reverse(que.pop_front().unwrap()));
            }
        }
    }
}