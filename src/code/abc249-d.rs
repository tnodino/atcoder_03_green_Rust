// https://atcoder.jp/contests/abc249/tasks/abc249_d

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    let ma = *A.iter().max().unwrap();
    let mut ans: usize = 0;
    for k in 1..=ma {
        for j in 1..=(ma / k) {
            let i = k * j;
            ans += map.get(&i).map_or(0, |&x| x) * map.get(&j).map_or(0, |&x| x) * map.get(&k).map_or(0, |&x| x)
        }
    }
    println!("{}", ans);
}