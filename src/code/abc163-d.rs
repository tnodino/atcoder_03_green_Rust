// https://atcoder.jp/contests/abc163/tasks/abc163_d

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut l = 0;
    let mut r = 0;
    for k in 0..K {
        l += k;
        r += N - k;
    }
    let mut ans = 0;
    for k in K..=N+1 {
        ans += r + 1 - l;
        ans %= MOD;
        if k == N + 1 {
            break;
        }
        l += k;
        r += N - k;
    }
    println!("{}", ans);
}