// https://atcoder.jp/contests/abc111/tasks/arc103_a

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for i in (0..n).step_by(2) {
        *map1.entry(v[i]).or_insert(0) += 1;
        *map2.entry(v[i+1]).or_insert(0) += 1;
    }
    let mut vec1 = map1.into_iter().map(|(k, v)| (v, k)).collect::<Vec<(usize, usize)>>();
    let mut vec2 = map2.into_iter().map(|(k, v)| (v, k)).collect::<Vec<(usize, usize)>>();
    vec1.sort_by(|a, b| b.0.cmp(&a.0));
    vec2.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = n;
    if vec1[0].1 != vec2[0].1 {
        ans = min(ans, n - vec1[0].0 - vec2[0].0);
    }
    if vec1.len() == 1 && vec2.len() == 1 {
        ans = min(ans, n / 2);
    }
    if vec1.len() >= 2 {
        ans = min(ans, n - vec1[1].0 - vec2[0].0);
    }
    if vec2.len() >= 2 {
        ans = min(ans, n - vec1[0].0 - vec2[1].0);
    }
    println!("{}", ans);
}