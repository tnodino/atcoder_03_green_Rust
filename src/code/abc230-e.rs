// https://atcoder.jp/contests/abc230/tasks/abc230_e

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut ans = 0;
    for i in 1..=N/(M+1) {
        ans += N / i;
    }
    for i in 1..=M {
        ans += (N / i - N / (i + 1)) * i;
    }
    println!("{}", ans);
}