// https://atcoder.jp/contests/abc153/tasks/abc153_e

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut DP = vec![INF; H+1];
    DP[0] = 0;
    for i in 1..=H {
        for j in 0..N {
            if i <= A[j] {
                DP[i] = min(DP[i], B[j]);
            }
            else {
                DP[i] = min(DP[i], DP[i-A[j]] + B[j]);
            }
        }
    }
    println!("{}", DP[H]);
}