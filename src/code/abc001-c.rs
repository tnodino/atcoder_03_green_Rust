// https://atcoder.jp/contests/abc001/tasks/abc001_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut Deg: f64,
        mut Dis: f64,
    }
    Deg /= 10.;
    let l = [ 0.00, 11.25, 33.75, 56.25,  78.75, 101.25, 123.75, 146.25, 168.75, 191.25, 213.75, 236.25, 258.75, 281.25, 303.75, 326.25, 348.75];
    let r = [11.25, 33.75, 56.25, 78.75, 101.25, 123.75, 146.25, 168.75, 191.25, 213.75, 236.25, 258.75, 281.25, 303.75, 326.25, 348.75, 360.00];
    let d = ["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW", "N"];
    let mut Dir = "";
    for i in 0..l.len() {
        if l[i] <= Deg && Deg < r[i] {
            Dir = d[i];
            break;
        }
    }
    Dis /=  60.;
    Dis = (Dis * 10.).round() / 10.;
    let l = [0.0, 0.3, 1.6, 3.4, 5.5,  8.0, 10.8, 13.9, 17.2, 20.8, 24.5, 28.5,    32.7];
    let r = [0.2, 1.5, 3.3, 5.4, 7.9, 10.7, 13.8, 17.1, 20.7, 24.4, 28.4, 32.6, 12000.0];
    let mut W = 0;
    for i in 0..l.len() {
        if l[i] <= Dis && Dis <= r[i] {
            W = i;
            break;
        }
    }
    if W == 0 {
        Dir = "C";
    }
    println!("{} {}", Dir, W);
}