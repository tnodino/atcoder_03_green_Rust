// https://atcoder.jp/contests/abc246/tasks/abc246_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

fn f(a: isize, b: isize) -> isize {
    return a * a * a + a * a * b + a * b * b + b * b * b;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
    }
    let M = 1_000_000;
    let mut j = M;
    let mut ans = f(M, 0);
    for i in 0..=M {
        while j >= 0 && f(i, j) >= N {
            ans = min(ans, f(i, j));
            j -= 1;
        }
    }
    println!("{}", ans);
}