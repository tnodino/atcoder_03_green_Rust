// https://atcoder.jp/contests/abc051/tasks/abc051_c

use proconio::input;
use proconio::fastout;

const DX: [char; 2] = ['R', 'L'];
const DY: [char; 2] = ['U', 'D'];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize,
    }
    let X = tx - sx;
    let Y = ty - sy;
    let mut flg_x = 0;
    let mut flg_y = 0;
    for _ in 0..X {
        print!("{}", DX[flg_x]);
    }
    for _ in 0..Y {
        print!("{}", DY[flg_y]);
    }
    flg_x ^= 1;
    flg_y ^= 1;
    for _ in 0..X {
        print!("{}", DX[flg_x]);
    }
    for _ in 0..Y {
        print!("{}", DY[flg_y]);
    }
    flg_x ^= 1;
    flg_y ^= 1;
    print!("D");
    for _ in 0..=X {
        print!("{}", DX[flg_x]);
    }
    for _ in 0..=Y {
        print!("{}", DY[flg_y]);
    }
    flg_x ^= 1;
    flg_y ^= 1;
    print!("LU");
    for _ in 0..=X {
        print!("{}", DX[flg_x]);
    }
    for _ in 0..=Y {
        print!("{}", DY[flg_y]);
    }
    println!("R");
}