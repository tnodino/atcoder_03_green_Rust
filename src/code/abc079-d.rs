// https://atcoder.jp/contests/abc079/tasks/abc079_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        mut c: [[usize; 10]; 10],
        A: [[isize; W]; H],
    }
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                c[i][j] = min(c[i][j], c[i][k] + c[k][j]);
            }
        }
    }
    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if A[i][j] == -1 {
                continue;
            }
            let idx = A[i][j] as usize;
            ans += c[idx][1];
        }
    }
    println!("{}", ans);
}