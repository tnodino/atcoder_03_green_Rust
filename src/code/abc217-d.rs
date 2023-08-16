// https://atcoder.jp/contests/abc217/tasks/abc217_d

use proconio::input;
use proconio::fastout;
use std::collections::BTreeSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        Q: usize,
    }
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(L);
    for _ in 0..Q {
        input! {
            c: usize,
            x: usize,
        }
        if c == 1 {
            set.insert(x);
        }
        else {
            let l = set.range(..x).next_back().unwrap();
            let r = set.range(x+1..).next().unwrap();
            println!("{}", r - l);
        }
    }
}