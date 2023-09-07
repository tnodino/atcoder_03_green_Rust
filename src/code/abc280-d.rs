// https://atcoder.jp/contests/abc280/tasks/abc280_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut K: usize,
    }
    let N = sqrt(K as f64) as usize + 1;
    let mut ans = 1;
    for i in 2..=N {
        let mut cnt = 0;
        while K % i == 0 {
            K /= i;
            cnt += 1;
        }
        let mut mul = 0;
        while cnt > 0 {
            mul += i;
            let mut k = mul;
            while k % i == 0 {
                k /= i;
                cnt -= 1;
            }
        }
        ans = max(ans, mul);
    }
    ans = max(ans, K);
    println!("{}", ans);
}