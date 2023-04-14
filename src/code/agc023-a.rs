// https://atcoder.jp/contests/agc023/tasks/agc023_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut S = vec![0; N+1];
    for i in 0..N {
        S[i+1] += S[i] + A[i];
    }
    let mut map: HashMap<isize, usize> = HashMap::new();
    for s in S {
        *map.entry(s).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    for v in map.values() {
        ans += (v - 1) * v / 2;
    }
    println!("{}", ans);
}