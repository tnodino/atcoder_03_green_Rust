// https://atcoder.jp/contests/abc296/tasks/abc296_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize
    }
    let L = sqrt(M as f64) as usize + 1;
    let mut ans: usize = !0;
    for a in 1..=L {
        let b = (M + a - 1) / a;
        if a <= N && 1 <= b && b <= N {
            ans = min(ans, a * b);
        }
    }
    if ans == !0 {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}