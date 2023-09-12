// https://atcoder.jp/contests/abc194/tasks/abc194_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let n = N as f64;
    let mut ans = 0.;
    for i in 1..N {
        ans += n / i as f64;
    }
    println!("{}", ans);
}