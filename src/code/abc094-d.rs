// https://atcoder.jp/contests/abc094/tasks/arc095_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }
    a.sort();
    let mut r = 0;
    for i in 0..n-1 {
        if ((a[n-1] + 1) / 2 - a[i]).abs() < ((a[n-1] + 1) / 2 - r).abs() {
            r = a[i];
        }
    }
    println!("{} {}", a[n-1], r);
}