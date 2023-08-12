// https://atcoder.jp/contests/abc077/tasks/arc084_a

use proconio::input;
use proconio::fastout;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N],
        mut C: [usize; N],
    }
    A.sort();
    B.sort();
    C.sort();
    let mut ans = 0;
    for i in 0..N {
        let l = bisect_left(&A, &B[i]);
        let r = N - bisect_left(&C, &(B[i]+1));
        ans += l * r;
    }
    println!("{}", ans);
}