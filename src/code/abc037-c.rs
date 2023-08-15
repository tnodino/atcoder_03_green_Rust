// https://atcoder.jp/contests/abc037/tasks/abc037_c

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
    let mut s = 0;
    for i in 0..K {
        s += a[i];
    }
    let mut ans = s;
    for i in K..N {
        s -= a[i-K];
        s += a[i];
        ans += s;
    }
    println!("{}", ans);
}