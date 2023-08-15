// https://atcoder.jp/contests/abc099/tasks/abc099_c

use proconio::input;
use proconio::fastout;
use num::pow;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut DP = vec![INF; N+1];
    DP[0] = 0;
    for i in 1..=N {
        DP[i] = min(DP[i], DP[i-1] + 1);
        for k in 1..10 {
            if pow(6, k) <= i {
                let x = pow(6, k);
                DP[i] = min(DP[i], DP[i-x] + 1);
            }
        }
        for k in 1..10 {
            if pow(9, k) <= i {
                let x = pow(9, k);
                DP[i] = min(DP[i], DP[i-x] + 1);
            }
        }
    }
    println!("{}", DP[N]);
}