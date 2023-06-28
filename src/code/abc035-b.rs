// https://atcoder.jp/contests/abc035/tasks/abc035_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: usize,
    }
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut cnt = 0;
    for s in S.chars() {
        match s {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => cnt += 1,
        }
    }
    if T == 1 {
        println!("{}", x.abs() + y.abs() + cnt);
    }
    else {
        if x.abs() + y.abs() >= cnt {
            println!("{}", x.abs() + y.abs() - cnt);
        }
        else {
            println!("{}", (cnt - (x.abs() + y.abs())) % 2);
        }
    }
}