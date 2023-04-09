// https://atcoder.jp/contests/agc017/tasks/agc017_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use num::pow;

#[allow(non_snake_case)]
fn ncr(n: usize, r: usize) -> usize {
    let r = min(r, n - r);
    let M = n - r + 1;
    let mut ret = 1;
    for x in M..=n {
        ret *= x;
    }
    for i in 1..=r {
        ret /= i;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        A: [usize; N],
    }
    let mut odd = 0;
    let mut even = 0;
    for a in A {
        if a % 2 == 0 {
            even += 1;
        }
        else {
            odd += 1;
        }
    }
    let mut ans = 0;
    let p = pow(2, even);
    if P == 0 {
        for k in (0..=odd).step_by(2) {
            ans += ncr(odd, k) * p;
        }
    }
    else {
        for k in (1..=odd).step_by(2) {
            ans += ncr(odd, k) * p;
        }
    }
    println!("{}", ans);
}