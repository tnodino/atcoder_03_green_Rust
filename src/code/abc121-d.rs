// https://atcoder.jp/contests/abc121/tasks/abc121_d

use proconio::input;
use proconio::fastout;

fn f(x: usize) -> usize {
    let mut ret = (x + 1) / 2 % 2;
    if x % 2 == 0 {
        ret ^= x;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", f(A-1) ^ f(B));
}