// https://atcoder.jp/contests/abc301/tasks/abc301_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        N: usize,
    }
    let M = S.len();
    let mut S = S.chars().collect::<Vec<char>>();
    S.reverse();
    let mut s: usize = 0;
    for i in (0..M).rev() {
        if S[i] == '1' {
            s += (1 as usize) << i;
        }
    }
    if s > N {
        println!("-1");
        return;
    }
    for i in (0..M).rev() {
        if S[i] == '?' && s + ((1 as usize) << i) <= N {
            s += (1 as usize) << i;
        }
    }
    println!("{}", s);
}