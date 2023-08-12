// https://atcoder.jp/contests/abc127/tasks/abc127_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
    }
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            B: usize,
            C: usize,
        }
        vec.push((C, B));
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    let mut cnt = 0;
    for i in 0..M {
        cnt += vec[i].1;
        for _ in 0..vec[i].1 {
            A.push(vec[i].0);
        }
        if cnt > N {
            break;
        }
    }
    A.sort_by(|a, b| b.cmp(&a));
    println!("{}", A[..N].iter().sum::<usize>());
}