// https://atcoder.jp/contests/abc245/tasks/abc245_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; N+1],
        mut C: [isize; N+M+1],
    }
    let mut B = vec![0; M+1];
    for i in (0..=M).rev() {
        B[i] = C[N+i] / A[N];
        for j in 0..=N {
            C[i+j] -= B[i] * A[j];
        }
    }
    println!("{}", B.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}