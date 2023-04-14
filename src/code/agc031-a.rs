// https://atcoder.jp/contests/agc031/tasks/agc031_a

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut cnt = [1; 26];
    for s in S.chars() {
        let idx = s as usize - 97;
        cnt[idx] += 1;
    }
    let mut ans = 1;
    for i in 0..26 {
        ans *= cnt[i];
        ans %= MOD;
    }
    println!("{}", (ans + MOD - 1) % MOD);
}