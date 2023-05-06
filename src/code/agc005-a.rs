// https://atcoder.jp/contests/agc005/tasks/agc005_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
    }
    let mut s = 0;
    let mut t = 0;
    for x in X.chars() {
        if x == 'S' {
            s += 1;
        }
        else {
            if s >= 1 {
                s -= 1;
            }
            else {
                t += 1;
            }
        }
    }
    println!("{}", s + t);
}