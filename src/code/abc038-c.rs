// https://atcoder.jp/contests/abc038/tasks/abc038_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    let mut l = 0;
    for i in 1..N {
        if a[i-1] >= a[i] {
            let m = i - l;
            ans += (m + 1) * m / 2;
            l = i;
        }
    }
    let m = N - l;
    ans += (m + 1) * m / 2;
    println!("{}", ans);
}