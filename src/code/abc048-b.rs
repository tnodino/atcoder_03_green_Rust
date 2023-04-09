// https://atcoder.jp/contests/abc048/tasks/abc048_b

use proconio::input;
use proconio::fastout;

fn f(n: isize, x: isize) -> isize {
    if n == -1 {
        return 0;
    }
    return n / x + 1;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        x: isize,
    }
    println!("{}", f(b, x) - f(a - 1, x));
}