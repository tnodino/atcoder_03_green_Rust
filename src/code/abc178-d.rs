// https://atcoder.jp/contests/abc178/tasks/abc178_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
    }
    let mut DP = vec![0; S+1];
    let mut s = 0;
    for i in 3..=S {
        s += DP[i-3];
        s %= MOD;
        DP[i] = s + 1;
        DP[i] %= MOD;
    }
    println!("{}", DP[S]);
}