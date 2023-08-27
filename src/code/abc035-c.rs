// https://atcoder.jp/contests/abc035/tasks/abc035_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut s = vec![0; N+1];
    for _ in 0..Q {
        input! {
            l: usize,
            r: usize,
        }
        s[l-1] += 1;
        s[r] -= 1;
    }
    for i in 0..N {
        print!("{}", s[i] % 2);
        s[i+1] += s[i];
    }
    println!();
}