// https://atcoder.jp/contests/abc156/tasks/abc156_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut xa = 1;
    let mut xb = 1;
    for i in 1..=a {
        xa *= n + 1 - i;
        xb *= i;
        xa %= MOD;
        xb %= MOD;
    }
    let x = xa * bin_power(xb, MOD-2) % MOD;
    let mut ya = 1;
    let mut yb = 1;
    for i in 1..=b {
        ya *= n + 1 - i;
        yb *= i;
        ya %= MOD;
        yb %= MOD;
    }
    let y = ya * bin_power(yb, MOD-2) % MOD;
    println!("{}", (bin_power(2, n) + MOD * 10 - x - y - 1) % MOD);
}