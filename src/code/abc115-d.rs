// https://atcoder.jp/contests/abc115/tasks/abc115_d

use proconio::input;
use proconio::fastout;

fn f(n: usize, x: isize, num: &Vec<isize>, pate: &Vec<isize>) -> isize {
    if n == 0 {
        return match x <= 0 {
            true => 0,
            false => 1,
        };
    }
    return match x <= num[n-1] + 1 {
        true => f(n - 1, x - 1, &num, &pate),
        false => pate[n-1] + 1 + f(n - 1, x - num[n-1] - 2, &num, &pate),
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X): (usize, isize),
    }
    let mut num = vec![1; N+1];
    let mut pate = vec![1; N+1];
    for i in 1..=N {
        num[i] = num[i-1] * 2 + 3;
        pate[i] = pate[i-1] * 2 + 1;
    }
    println!("{}", f(N, X, &num, &pate));
}