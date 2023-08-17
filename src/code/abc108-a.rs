// https://atcoder.jp/contests/abc108/tasks/arc102_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    if K % 2 == 0 {
        let mut cnt1: usize = 0;
        let mut cnt2: usize = 0;
        for i in 1..=N {
            if i % K == 0 {
                cnt1 += 1;
            }
            if i % K == K / 2 {
                cnt2 += 1;
            }
        }
        println!("{}", cnt1 * cnt1 * cnt1 + cnt2 * cnt2 * cnt2);
    }
    else {
        let mut cnt: usize = 0;
        for i in 1..=N {
            if i % K == 0 {
                cnt += 1;
            }
        }
        println!("{}", cnt * cnt * cnt);
    }
}