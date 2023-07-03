// https://atcoder.jp/contests/abc125/tasks/abc125_c

use proconio::input;
use proconio::fastout;
use num::integer::gcd;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut L = vec![0; N+1];
    let mut R = vec![0; N+1];
    for i in 0..N {
        L[i+1] = gcd(L[i], A[i]);
    }
    for i in (1..=N).rev() {
        R[i-1] = gcd(R[i], A[i-1]);
    }
    let mut ans = 0;
    for i in 0..N {
        ans = max(ans, gcd(L[i], R[i+1]));
    }
    println!("{}", ans);
}