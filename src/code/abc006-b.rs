// https://atcoder.jp/contests/abc006/tasks/abc006_2

use proconio::input;
use proconio::fastout;
use ac_library::ModInt as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let m = 1_000_000;
    Mint::set_modulus(10007);
    let mut DP = vec![Mint::new(0); m+1];
    DP[3] = 1.into();
    for i in 4..=n {
        DP[i] = DP[i-1] + DP[i-2] + DP[i-3];
    }
    println!("{}", DP[n]);
}