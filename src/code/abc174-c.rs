// https://atcoder.jp/contests/abc174/tasks/abc174_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
    }
    let mut x = 0;
    for i in 1..=K {
        x = ((x * 10) + 7) % K;
        if x == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}