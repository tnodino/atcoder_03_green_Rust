// https://atcoder.jp/contests/abc011/tasks/abc011_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: isize,
        NG1: isize,
        NG2: isize,
        NG3: isize,
    }
    if NG1 == N || NG2 == N || NG3 == N {
        println!("NO");
        return;
    }
    for _ in 0..100 {
        for k in (1..=3).rev() {
            let x = N - k;
            if NG1 == x || NG2 == x || NG3 == x {
                continue;
            }
            N = x;
            break;
        }
    }
    if N <= 0 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}