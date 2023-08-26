// https://atcoder.jp/contests/abc252/tasks/abc252_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 200_000;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    for i in 0..M {
        cnt[i+1] += cnt[i];
    }
    let mut ans: usize = 0;
    for i in 0..N {
        ans += cnt[A[i]-1] * (cnt[M] - cnt[A[i]]);
    }
    println!("{}", ans);
}