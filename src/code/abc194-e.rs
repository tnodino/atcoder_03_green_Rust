// https://atcoder.jp/contests/abc194/tasks/abc194_e

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
    }
    let mut idx = vec![vec![0]; N];
    for i in 0..N {
        idx[A[i]].push(i + 1);
    }
    for i in 0..N {
        idx[i].push(N + 1);
        for j in 0..idx[i].len()-1 {
            if idx[i][j+1] - idx[i][j] > M {
                println!("{}", i);
                return;
            }
        }
    }
    println!("{}", N);
}