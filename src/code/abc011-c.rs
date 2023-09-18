// https://atcoder.jp/contests/abc011/tasks/abc011_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut N, NG1, NG2, NG3): (isize, isize, isize, isize),
    }
    if N == NG1 || N == NG2 || N == NG3 {
        println!("NO");
        return;
    }
    'cont: for _ in 0..100 {
        for x in (1..=3).rev() {
            if N - x != NG1 && N - x != NG2 && N - x != NG3 {
                N -= x;
                continue 'cont;
            }
        }
        println!("NO");
        return;
    }
    println!("{}", match N <= 0 {
        true => "YES",
        false => "NO",
    })
}