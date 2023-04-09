// https://atcoder.jp/contests/code-festival-2017-quala/tasks/code_festival_2017_quala_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    for i in 0..=N {
        for j in 0..=M {
            if M * i + N * j - i * j * 2 == K {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}