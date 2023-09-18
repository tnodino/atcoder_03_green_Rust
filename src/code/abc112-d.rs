// https://atcoder.jp/contests/abc112/tasks/abc112_d

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let m = sqrt(M as f64) as usize;
    let mut vec = Vec::new();
    for i in 1..=m {
        if M % i == 0 {
            vec.push(i);
            vec.push(M / i);
        }
    }
    vec.sort_by(|a, b| b.cmp(&a));
    for i in 0..vec.len() {
        if vec[i] * N <= M {
            println!("{}", vec[i]);
            return;
        }
    }
}