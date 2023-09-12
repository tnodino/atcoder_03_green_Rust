// https://atcoder.jp/contests/abc293/tasks/abc293_d

use proconio::input;
use proconio::fastout;
use ac_library::Dsu;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }
    let mut UF = Dsu::new(N);
    let mut cnt = 0;
    for _ in 0..M {
        input! {
            (A, _, C, _): (usize, char, usize, char),
        }
        if UF.same(A-1, C-1) {
            cnt += 1;
        }
        UF.merge(A-1, C-1);
    }
    let group = (0..N).filter(|&x| UF.leader(x) == x).count();
    println!("{} {}", cnt, group - cnt);
}