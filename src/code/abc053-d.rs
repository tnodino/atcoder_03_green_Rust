// https://atcoder.jp/contests/abc053/tasks/arc068_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.dedup();
    let M = A.len();
    if M % 2 == 0 {
        println!("{}", M - 1);
    }
    else {
        println!("{}", M);
    }
}