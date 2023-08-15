// https://atcoder.jp/contests/abc028/tasks/abc028_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: f64,
        K: f64,
    }
    let a = (K - 1.) * (N - K) * 6.;
    let b = ((K - 1.) + (N - K)) * 3.;
    let c = 1.;
    let n = N * N * N;
    println!("{}", (a + b + c) / n);
}