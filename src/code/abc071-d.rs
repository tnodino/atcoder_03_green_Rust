// https://atcoder.jp/contests/abc071/tasks/arc081_b

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S1: String,
        S2: String,
    }
    let S1 = S1.chars().collect::<Vec<char>>();
    let S2 = S2.chars().collect::<Vec<char>>();
    let mut ans;
    if S1[0] == S2[0] {
        ans = 3;
    }
    else {
        ans = 6;
    }
    for i in 1..N {
        if S1[i-1] == S1[i] {
            continue;
        }
        if S1[i-1] == S2[i-1] {
            ans *= 2;
        }
        else if S1[i] != S2[i] {
            ans *= 3;
        }
        ans %= MOD;
    }
    println!("{}", ans);
}