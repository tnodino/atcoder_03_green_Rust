// https://atcoder.jp/contests/abc006/tasks/abc006_2

use proconio::input;
use proconio::fastout;

const MOD: usize = 10_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    if n <= 3 {
        if n <= 2 {
            println!("0");
        }
        else {
            println!("1");
        }
        return
    }
    let mut DP = vec![0; n+1];
    DP[3] = 1;
    for i in 4..=n {
        DP[i] += DP[i-1] + DP[i-2] + DP[i-3];
        DP[i] %= MOD;
    }
    println!("{}", DP[n]);
}