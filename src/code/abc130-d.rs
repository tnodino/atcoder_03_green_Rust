// https://atcoder.jp/contests/abc130/tasks/abc130_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; N],
    }
    let mut ans = (N + 1) * N / 2;
    let mut now = 0;
    let mut l = 0;
    for r in 0..N {
        now += a[r];
        while l <= r && now >= K {
            now -= a[l];
            l += 1;
        }
        ans -= r + 1 - l;
    }
    println!("{}", ans);
}