// https://atcoder.jp/contests/abc160/tasks/abc160_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        A: usize,
        B: usize,
        C: usize,
        mut p: [usize; A],
        mut q: [usize; B],
        mut r: [usize; C],
    }
    p.sort_by(|a, b| b.cmp(&a));
    q.sort_by(|a, b| b.cmp(&a));
    for i in 0..X {
        r.push(p[i]);
    }
    for i in 0..Y {
        r.push(q[i]);
    }
    r.sort_by(|a, b| b.cmp(&a));
    println!("{}", r[..X+Y].iter().sum::<usize>());
}