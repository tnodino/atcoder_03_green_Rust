// https://atcoder.jp/contests/abc197/tasks/abc197_d

use proconio::input;
use proconio::fastout;
use libm::{hypot, sin, cos, atan2};
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: f64,
        x0: f64,
        y0: f64,
        xn: f64,
        yn: f64,
    }
    let ox = (x0 + xn) / 2.;
    let oy = (y0 + yn) / 2.;
    let zx = x0 - ox;
    let zy = y0 - oy;
    let r = hypot(zx, zy);
    let rad = atan2(zy, zx) + 2. * PI / N;
    let x1 = r * cos(rad) + ox;
    let y1 = r * sin(rad) + oy;
    println!("{} {}", x1, y1);
}