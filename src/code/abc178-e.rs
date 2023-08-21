// https://atcoder.jp/contests/abc178/tasks/abc178_e

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            x: isize,
            y: isize,
        }
        vec.push((x + y, x - y));
    }
    vec.sort_by(|a, b|
    if a.0 != b.0 {
        a.0.cmp(&b.0)
    }
    else {
        a.1.cmp(&b.1)
    });
    let ans1 = max((vec[0].0 - vec[N-1].0).abs(), (vec[0].1 - vec[N-1].1).abs());
    vec.sort_by(|a, b|
    if a.1 != b.1 {
        a.1.cmp(&b.1)
    }
    else {
        a.0.cmp(&b.0)
    });
    let ans2 = max((vec[0].0 - vec[N-1].0).abs(), (vec[0].1 - vec[N-1].1).abs());
    println!("{}", max(ans1, ans2));
}