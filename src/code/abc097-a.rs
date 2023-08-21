// https://atcoder.jp/contests/abc097/tasks/arc097_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        K: usize,
    }
    let N = s.len();
    let mut vec = Vec::new();
    for i in 0..N {
        for j in 1..=K {
            if i + j <= N {
                vec.push(&s[i..i+j]);
            }
        }
    }
    vec.sort();
    vec.dedup();
    println!("{}", vec[K-1]);
}