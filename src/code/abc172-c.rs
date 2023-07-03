// https://atcoder.jp/contests/abc172/tasks/abc172_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

use std::cmp::Ordering;

fn bisect_right<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x <= v {
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
        M: usize,
        K: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut SA = vec![0; N+1];
    for i in 0..N {
        SA[i+1] = SA[i] + A[i];
    }
    let mut SB = vec![0; M];
    SB[0] = B[0];
    for i in 1..M {
        SB[i] = SB[i-1] + B[i];
    }
    let mut ans = 0;
    for i in 0..=N {
        if SA[i] > K {
            break;
        }
        let x = bisect_right(&SB, &(K-SA[i]));
        ans = max(ans, i+x);
    }
    println!("{}", ans);
}