// https://atcoder.jp/contests/agc002/tasks/agc002_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut cnt = vec![1; N];
    let mut flg = vec![false; N];
    flg[0] = true;
    for _ in 0..M {
        input! {
            (x, y): (usize, usize),
        }
        let x = x - 1;
        let y = y - 1;
        cnt[x] -= 1;
        cnt[y] += 1;
        flg[y] |= flg[x];
        if cnt[x] == 0 {
            flg[x] = false;
        }
    }
    println!("{}", flg.iter().filter(|&x| *x).count());
}