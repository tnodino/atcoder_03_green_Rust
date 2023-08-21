// https://atcoder.jp/contests/abc188/tasks/abc188_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        C: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        vec.push((a, c, 1));
        vec.push((b + 1, c, 2));
    }
    vec.sort();
    let mut ans = 0;
    let mut l = 0;
    let mut cost = 0;
    for i in 0..N*2 {
        if l < vec[i].0 {
            ans += (vec[i].0 - l) * min(cost, C);
            l = vec[i].0;
        }
        match vec[i].2 {
            1 => cost += vec[i].1,
            _ => cost -= vec[i].1,
        }
    }
    println!("{}", ans);
}