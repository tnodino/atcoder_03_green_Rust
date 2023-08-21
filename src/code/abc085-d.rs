// https://atcoder.jp/contests/abc085/tasks/abc085_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut H: usize,
    }
    let mut a = 0;
    let mut b = vec![0];
    for _ in 0..N {
        input! {
            u: usize,
            v: usize,
        }
        a = max(a, u);
        b.push(v);
    }
    b.sort_by(|a, b| b.cmp(&a));
    for i in 0..=N {
        if b[i] < a {
            println!("{}", i + (H + a - 1) / a);
            return;
        }
        else {
            if H <= b[i] {
                println!("{}", i + 1);
                return;
            }
            else {
                H -= b[i];
            }
        }
    }
}