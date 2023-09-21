// https://atcoder.jp/contests/abc099/tasks/abc099_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = N;
    for i in 0..=N {
        let mut res = 0;
        let mut x = i;
        while x > 0 {
            res += x % 6;
            x /= 6;
        }
        let mut x = N - i;
        while x > 0 {
            res += x % 9;
            x /= 9;
        }
        ans = min(ans, res);
    }
    println!("{}", ans);
}