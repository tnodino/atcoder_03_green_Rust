// https://atcoder.jp/contests/abc037/tasks/abc037_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        a: [usize; N],
    }
    let mut sum = 0;
    for i in 0..K {
        sum += a[i];
    }
    let mut ans = sum;
    for i in K..N {
        sum -= a[i-K];
        sum += a[i];
        ans += sum;
    }
    println!("{}", ans);
}