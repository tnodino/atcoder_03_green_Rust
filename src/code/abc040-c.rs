// https://atcoder.jp/contests/abc040/tasks/abc040_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N],
    }
    let mut DP = vec![INF; N];
    DP[0] = 0;
    for i in 1..N {
        DP[i] = DP[i-1] + (a[i] - a[i-1]).abs();
        if 2 <= i {
            DP[i] = min(DP[i], DP[i-2] + (a[i] - a[i-2]).abs());
        }
    }
    println!("{}", DP[N-1]);
}