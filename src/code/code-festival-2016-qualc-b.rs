// https://atcoder.jp/contests/code-festival-2016-qualc/tasks/codefestival_2016_qualC_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: isize,
        T: usize,
        mut a: [isize; T],
    }
    a.sort();
    let N = a[T-1];
    let M = K - N + 1;
    println!("{}", max(0, N - M));
}