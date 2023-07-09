// https://atcoder.jp/contests/abc160/tasks/abc160_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: isize,
        Y: isize,
    }
    let mut ans = vec![0; N];
    for i in 1..=N {
        for j in i+1..=N {
            let a = i as isize;
            let b = j as isize;
            let idx = min(b - a, min((X-a).abs() + (b-Y).abs(), (Y-a).abs() + (b-X).abs()) + 1) as usize;
            ans[idx] += 1;
        }
    }
    for i in 1..N {
        println!("{}", ans[i]);
    }
}