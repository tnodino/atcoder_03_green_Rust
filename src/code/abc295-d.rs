// https://atcoder.jp/contests/abc295/tasks/abc295_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut g = 0;
    *map.entry(0).or_insert(0) += 1;
    for s in S {
        let num = s as usize - 48;
        g ^= 1 << num;
        *map.entry(g).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    for v in map.values() {
        ans += (v - 1) * v / 2;
    }
    println!("{}", ans);
}