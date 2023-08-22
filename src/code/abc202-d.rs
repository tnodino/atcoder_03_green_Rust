// https://atcoder.jp/contests/abc202/tasks/abc202_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
        mut K: usize,
    }
    let mut DP = vec![vec![0; B+1]; A+1];
    DP[0][0] = 1;
    for i in 0..=A {
        for j in 0..=B {
            if i > 0 {
                DP[i][j] += DP[i-1][j];
            }
            if j > 0 {
                DP[i][j] += DP[i][j-1];
            }
        }
    }
    while A > 0 && B > 0 {
        if K <= DP[A-1][B] {
            print!("a");
            A -= 1;
        }
        else {
            K -= DP[A-1][B];
            print!("b");
            B -= 1;
        }
    }
    print!("{}", "a".repeat(A));
    print!("{}", "b".repeat(B));
    println!();
}