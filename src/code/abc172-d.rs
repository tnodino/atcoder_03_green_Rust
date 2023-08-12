// https://atcoder.jp/contests/abc172/tasks/abc172_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = vec![0; N+1];
    let mut ans = 0;
    for i in 1..=N {
        for j in (i..=N).step_by(i) {
            cnt[j] += 1;
        }
        ans += cnt[i] * i;
    }
    println!("{}", ans);
}