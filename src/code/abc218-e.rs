// https://atcoder.jp/contests/abc218/tasks/abc218_e

use proconio::input;
use proconio::fastout;
use ac_library::Dsu;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            (A, B, C): (usize, usize, isize),
        }
        vec.push((C, A-1, B-1));
    }
    vec.sort();
    let mut UF = Dsu::new(N);
    let mut ans = 0;
    for i in 0..M {
        if 0 < vec[i].0 && UF.same(vec[i].1, vec[i].2) {
            ans += vec[i].0;
        }
        UF.merge(vec[i].1, vec[i].2);
    }
    println!("{}", ans);
}