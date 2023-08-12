// https://atcoder.jp/contests/abc148/tasks/abc148_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N % 2 == 1 {
        println!("0");
        return;
    }
    let mut ans = 0;
    let mut x = 10;
    for _ in 0..26 {
        ans += N / x;
        x *= 5;
    }
    println!("{}", ans);
}