// https://atcoder.jp/contests/abc052/tasks/arc067_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        X: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N-1 {
        ans += min((X[i+1] - X[i]) * A, B);
    }
    println!("{}", ans);
}