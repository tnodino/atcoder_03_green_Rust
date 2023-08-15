// https://atcoder.jp/contests/abc107/tasks/arc101_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        x: [isize; N],
    }
    let mut ans = 1<<50;
    for l in 0..=N-K {
        let r = l + K - 1;
        if x[r] <= 0 {
            ans = min(ans, x[l].abs());
        }
        else if 0 <= x[l] {
            ans = min(ans, x[r].abs());
        }
        else {
            ans = min(ans, min((x[l] * 2).abs() + x[r], x[l].abs() + (x[r] * 2)));
        }
    }
    println!("{}", ans);
}