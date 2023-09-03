// https://atcoder.jp/contests/abc238/tasks/abc238_d

use proconio::input;

#[allow(non_snake_case)]
fn solve() {
    input! {
        a: usize,
        s: usize,
    }
    if a * 2 > s {
        println!("No");
        return;
    }
    let t = s - 2 * a;
    if t & a == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}