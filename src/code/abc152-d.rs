// https://atcoder.jp/contests/abc152/tasks/abc152_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = vec![vec![0; 10]; 10];
    let mut ans: usize = 0;
    for i in 1..=N {
        let mut l = i;
        while l >= 10 {
            l /= 10;
        }
        let r = i % 10;
        cnt[l][r] += 1;
    }
    for i in 1..10 {
        for j in 1..10 {
            ans += cnt[i][j] * cnt[j][i];
        }
    }
    println!("{}", ans);
}