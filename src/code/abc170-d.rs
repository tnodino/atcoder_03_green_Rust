// https://atcoder.jp/contests/abc170/tasks/abc170_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 1_000_000;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    let mut ans = 0;
    for i in 1..=M {
        if cnt[i] == 0 {
            continue;
        }
        if cnt[i] == 1 {
            ans += 1;
        }
        for j in (i..=M).step_by(i) {
            cnt[j] = 0;
        }
    }
    println!("{}", ans);
}