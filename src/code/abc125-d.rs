// https://atcoder.jp/contests/abc125/tasks/abc125_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
    }
    let mut s = 0;
    let mut cnt = 0;
    for i in 0..N {
        if A[i] < 0 {
            cnt += 1;
        }
        A[i] = A[i].abs();
        s += A[i];
    }
    if cnt % 2 == 0 {
        println!("{}", s);
    }
    else {
        println!("{}", s - A.iter().min().unwrap() * 2);
    }
}