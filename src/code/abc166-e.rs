// https://atcoder.jp/contests/abc166/tasks/abc166_e

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
    let mut cntl: HashMap<isize, usize> = HashMap::new();
    let mut cntr: HashMap<isize, usize> = HashMap::new();
    for i in 0..N {
        let idx = (i + 1) as isize;
        *cntl.entry(idx + A[i]).or_insert(0) += 1;
        *cntr.entry(idx - A[i]).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    for (k, v) in cntl.iter() {
        ans += v * cntr.get(k).map_or(0, |&v| v);
    }
    println!("{}", ans);
}