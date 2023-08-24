// https://atcoder.jp/contests/abc140/tasks/abc140_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.dedup();
    let M = S.len();
    for i in 0..M-1 {
        if S[i] != S[i+1] {
            if K > 0 {
                S[i+1] = S[i];
                K -= 1;
            }
        }
    }
    S.dedup();
    println!("{}", N - S.len());
}