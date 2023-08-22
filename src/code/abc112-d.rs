// https://atcoder.jp/contests/abc112/tasks/abc112_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let K = sqrt(M as f64) as usize + 1;
    let mut vec = Vec::new();
    for i in 1..=K {
        if M % i == 0 {
            vec.push(i);
            vec.push(M / i);
        }
    }
    vec.sort();
    vec.dedup();
    for i in (0..vec.len()).rev() {
        if vec[i] * N <= M {
            println!("{}", vec[i]);
            return;
        }
    }
}