// https://atcoder.jp/contests/cf16-final/tasks/codefestival_2016_final_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut ma = 1;
    while (ma + 1) * ma / 2 < N {
        ma += 1;
    }
    for i in (1..=ma).rev() {
        if i <= N {
            println!("{}", i);
            N -= i;
        }
    }
}