// https://atcoder.jp/contests/abc057/tasks/abc057_c

use proconio::input;
use proconio::fastout;
use libm::sqrt;
use std::cmp::{min, max};

#[allow(non_snake_case)]
fn f(mut A: usize, mut B: usize) -> usize {
    let mut cnta = 0;
    let mut cntb = 0;
    while A > 0 {
        cnta += 1;
        A /= 10;
    }
    while B > 0 {
        cntb += 1;
        B /= 10;
    }
    return max(cnta, cntb);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans = N;
    for i in 1..=M {
        if N % i == 0 {
            let A = N / i;
            let B = i;
            ans = min(ans, f(A, B));
        }
    }
    println!("{}", ans);
}