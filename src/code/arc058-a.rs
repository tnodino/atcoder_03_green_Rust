// https://atcoder.jp/contests/arc058/tasks/arc058_a

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        D: [usize; K],
    }
    'outer: for n in N..=100_000 {
        let mut x = n;
        while x > 0 {
            let m = x % 10;
            if D.contains(&m) {
                continue 'outer;
            }
            x /= 10;
        }
        println!("{}", n);
        return;
    }
}