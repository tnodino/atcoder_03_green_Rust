// https://atcoder.jp/contests/abc271/tasks/abc271_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, mut S): (usize, usize),
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..N {
        input! {
            (i1, i2): (usize, usize),
        }
        a.push(i1);
        b.push(i2);
    }
    let mut DP = vec![vec![false; S+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        for j in 0..S {
            if j + a[i] <= S {
                DP[i+1][j+a[i]] |= DP[i][j];
            }
            if j + b[i] <= S {
                DP[i+1][j+b[i]] |= DP[i][j];
            }
        }
    }
    if !DP[N][S] {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ans = Vec::new();
    for i in (0..N).rev() {
        match a[i] <= S && DP[i][S-a[i]] {
            true => {
                ans.push('H');
                S -= a[i];
            },
            false => {
                ans.push('T');
                S -= b[i];
            },
        }
    }
    println!("{}", ans.iter().rev().collect::<String>());
}