// https://atcoder.jp/contests/abc204/tasks/abc204_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: [usize; N],
    }
    let M = T.iter().sum::<usize>();
    let mut DP = vec![vec![false; M+1]; N+1];
    DP[0][0] = true;
    for i in 1..=N {
        for j in 0..=M {
            DP[i][j] |= DP[i-1][j];
            if T[i-1] <= j {
                DP[i][j] |= DP[i-1][j-T[i-1]];
            }
        }
    }
    for i in (0..=M/2).rev() {
        if DP[N][i] {
            println!("{}", M - i);
            return;
        }
    }
}