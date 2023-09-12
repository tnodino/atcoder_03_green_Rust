// https://atcoder.jp/contests/abc080/tasks/abc080_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let M = 10;
    input! {
        N: usize,
        F: [[usize; M]; N],
        P: [[isize; M+1]; N]
    }
    let mut ans = -1<<50;
    for bit in 1..1<<M {
        let mut flg = vec![0; M];
        for i in 0..M {
            if bit & (1 << i) > 0 {
                flg[i] = 1;
            }
        }
        let mut cnt = vec![0; N];
        for i in 0..N {
            for j in 0..M {
                if F[i][j] & flg[j] == 1 {
                    cnt[i] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..N {
            res += P[i][cnt[i]];
        }
        ans = max(ans, res);
    }
    println!("{}", ans);
}