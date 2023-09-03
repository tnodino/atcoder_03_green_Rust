// https://atcoder.jp/contests/abc318/tasks/abc318_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut D = Vec::new();
    for i in 1..N {
        input! {
            d: [usize; N-i],
        }
        D.push(d);
    }
    let mut DP = vec![0; 1<<N];
    for bit in 0..1<<N {
        for i in 0..N-1 {
            if bit & (1 << i) == 0 {
                for j in i+1..N {
                    if bit & (1 << j) == 0 {
                        let bit2 = bit | (1 << i) | (1 << j);
                        DP[bit2] = max(DP[bit2], DP[bit] + D[i][j-i-1]);
                    }
                }
            }
        }
    }
    println!("{}", DP.iter().max().unwrap());
}