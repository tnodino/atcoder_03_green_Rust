// https://atcoder.jp/contests/abc234/tasks/abc234_e

use proconio::input;
use proconio::fastout;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    let mut vec = Vec::new();
    for i in 1..=9 {
        vec.push(i);
        for j in 0..=9 {
            let mut x = i;
            while x < X {
                let num = x % 10;
                if num + j >= 10 {
                    break;
                }
                x = x * 10 + num + j;
            }
            vec.push(x);
            let mut x = i;
            while x < X {
                let num = x % 10;
                if num < j {
                    break;
                }
                x = x * 10 + num - j;
            }
            vec.push(x);
        }
    }
    vec.sort();
    let idx = bisect_left(&vec, &X);
    println!("{}", vec[idx]);
}