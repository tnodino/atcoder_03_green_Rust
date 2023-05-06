// https://atcoder.jp/contests/abc048/tasks/arc064_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        x: usize,
        mut a: [usize; N],
    }
    let mut ans = 0;
    if a[0] > x {
        ans += a[0] - x;
        a[0] = x;
    }
    for i in 1..N {
        if a[i-1] + a[i] > x {
            let k = a[i-1] + a[i] - x;
            ans += k;
            a[i] -= k;
        }
    }
    println!("{}", ans);
}